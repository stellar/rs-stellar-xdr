//! XDR parser (recursive descent).

use crate::ast::*;
use crate::lexer::{IntBase, LexError, Lexer, SpannedToken, Token};
use heck::ToUpperCamelCase;
use thiserror::Error;

/// Parse XDR source text into an AST.
pub fn parse(source: &str) -> Result<XdrSpec, ParseError> {
    let mut parser = Parser::new(source)?;
    parser.parse()
}

pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
    /// The original source text, for extracting source snippets
    source: String,
    /// Track the token position of the current definition start for source extraction
    def_start_pos: usize,
    /// Extracted nested type definitions (anonymous unions, inline structs)
    extracted_definitions: Vec<Definition>,
    /// Root parent type name for generating nested type names
    root_parent: Option<String>,
    /// Global map of enum member names and const names to their values
    global_values: std::collections::HashMap<String, i64>,
}

impl Parser {
    pub fn new(source: &str) -> Result<Self, ParseError> {
        let lexer = Lexer::new(source);
        let (tokens, source) = lexer.tokenize_with_spans()?;
        Ok(Self {
            tokens,
            pos: 0,
            source,
            def_start_pos: 0,
            extracted_definitions: Vec::new(),
            root_parent: None,
            global_values: std::collections::HashMap::new(),
        })
    }

    /// Parse the entire input.
    pub fn parse(&mut self) -> Result<XdrSpec, ParseError> {
        let mut spec = XdrSpec::default();

        while *self.peek() != Token::Eof {
            // Skip any extra semicolons at the top level
            while *self.peek() == Token::Semi {
                self.advance();
            }
            if *self.peek() == Token::Eof {
                break;
            }
            match self.peek() {
                Token::Namespace => {
                    let ns = self.parse_namespace()?;
                    spec.namespaces.push(ns);
                }
                _ => {
                    // Track extracted definitions before parsing this definition
                    let extract_start = self.extracted_definitions.len();

                    let def = self.parse_definition()?;

                    // Insert any newly extracted definitions before this definition
                    // This ensures nested types appear just before their parent
                    for extracted in self.extracted_definitions.drain(extract_start..) {
                        spec.definitions.push(extracted);
                    }

                    spec.definitions.push(def);
                }
            }
        }

        // Any remaining extracted definitions (shouldn't be any, but just in case)
        for extracted in self.extracted_definitions.drain(..) {
            spec.definitions.push(extracted);
        }

        // Post-process to fix parent relationships for nested types.
        // For types whose name starts with another type's name, update parent to be that type.
        // This fixes cases where inline structs inside unions have nested unions.
        fix_parent_relationships(&mut spec.definitions);
        for ns in &mut spec.namespaces {
            fix_parent_relationships(&mut ns.definitions);
        }

        Ok(spec)
    }

    fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
        self.expect(Token::Namespace)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;

        let mut definitions = Vec::new();
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
            // Skip any extra semicolons
            while *self.peek() == Token::Semi {
                self.advance();
            }
            if *self.peek() == Token::RBrace {
                break;
            }

            // Track extracted definitions before parsing this definition
            let extract_start = self.extracted_definitions.len();

            let def = self.parse_definition()?;

            // Insert any newly extracted definitions before this definition
            // This ensures nested types appear just before their parent
            for extracted in self.extracted_definitions.drain(extract_start..) {
                definitions.push(extracted);
            }

            definitions.push(def);
        }

        self.expect(Token::RBrace)?;

        Ok(Namespace { name, definitions })
    }

    fn parse_definition(&mut self) -> Result<Definition, ParseError> {
        // Mark the start of this definition for source extraction
        self.def_start_pos = self.pos;

        match self.peek() {
            Token::Struct => self.parse_struct().map(Definition::Struct),
            Token::Enum => self.parse_enum().map(Definition::Enum),
            Token::Union => self.parse_union().map(Definition::Union),
            Token::Typedef => self.parse_typedef().map(Definition::Typedef),
            Token::Const => self.parse_const().map(Definition::Const),
            other => Err(self.unexpected_token_error(
                "struct, enum, union, typedef, or const".to_string(),
                other.clone(),
            )),
        }
    }

    fn parse_struct(&mut self) -> Result<Struct, ParseError> {
        self.expect(Token::Struct)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;

        // Set root_parent for nested type name generation
        let prev_root = self.root_parent.take();
        self.root_parent = Some(name.clone());

        let mut members = Vec::new();
        while *self.peek() != Token::RBrace {
            let member = self.parse_member()?;
            members.push(member);
            self.expect(Token::Semi)?;
        }

        // Restore previous root_parent
        self.root_parent = prev_root;

        self.expect(Token::RBrace)?;
        self.expect(Token::Semi)?;

        // Extract source text (simplified - just use the name for now)
        let source = self.extract_definition_source(&name);

        Ok(Struct {
            name,
            members,
            source,
            is_nested: false,
            parent: None,
        })
    }

    fn parse_enum(&mut self) -> Result<Enum, ParseError> {
        self.expect(Token::Enum)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;

        let mut members = Vec::new();
        loop {
            let member_name = self.expect_ident()?;
            self.expect(Token::Eq)?;

            // Value can be integer or identifier (reference to another enum value)
            let value = match self.peek().clone() {
                Token::IntLiteral((value, _)) => {
                    self.advance();
                    value as i32
                }
                Token::Ident(ref_name) => {
                    self.advance();
                    self.resolve_enum_value(&ref_name, &members)?
                }
                other => {
                    return Err(
                        self.unexpected_token_error("integer or identifier".to_string(), other)
                    )
                }
            };

            members.push(EnumMember {
                name: member_name,
                value,
            });

            match self.peek() {
                Token::Comma => {
                    self.advance();
                    if *self.peek() == Token::RBrace {
                        break;
                    }
                }
                Token::RBrace => break,
                other => {
                    return Err(self.unexpected_token_error(", or }".to_string(), other.clone()))
                }
            }
        }

        self.expect(Token::RBrace)?;
        self.expect(Token::Semi)?;

        let source = self.extract_definition_source(&name);

        // Add all members to global_values for cross-enum resolution
        for m in &members {
            self.global_values.insert(m.name.clone(), m.value as i64);
        }

        Ok(Enum {
            name,
            members,
            source,
        })
    }

    fn parse_union(&mut self) -> Result<Union, ParseError> {
        self.expect(Token::Union)?;
        let name = self.expect_ident()?;
        self.expect(Token::Switch)?;
        self.expect(Token::LParen)?;

        // Parse discriminant
        let disc_type = self.parse_type()?;
        let disc_name = self.expect_ident()?;

        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;

        // Set root_parent for inline struct extraction
        let prev_root = self.root_parent.take();
        self.root_parent = Some(name.clone());

        let mut arms = Vec::new();
        let mut default_arm = None;

        while *self.peek() != Token::RBrace {
            let arm = self.parse_union_arm()?;
            if arm.cases.is_empty() {
                // This is a default arm
                default_arm = Some(Box::new(arm));
            } else {
                arms.push(arm);
            }
        }

        // Restore previous root_parent
        self.root_parent = prev_root;

        self.expect(Token::RBrace)?;
        self.expect(Token::Semi)?;

        let source = self.extract_definition_source(&name);

        Ok(Union {
            name,
            discriminant: Discriminant {
                name: disc_name,
                type_: disc_type,
            },
            arms,
            default_arm,
            source,
            is_nested: false,
            parent: None,
        })
    }

    fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
        self.expect(Token::Typedef)?;

        let type_ = self.parse_type()?;
        let name = self.expect_ident()?;

        // Handle array suffix on typedef name
        let type_ = self.parse_type_suffix(type_)?;

        self.expect(Token::Semi)?;

        let source = self.extract_definition_source(&name);

        Ok(Typedef {
            name,
            type_,
            source,
        })
    }

    fn parse_const(&mut self) -> Result<Const, ParseError> {
        self.expect(Token::Const)?;
        let name = self.expect_ident()?;
        self.expect(Token::Eq)?;
        let (value, base) = self.expect_int_with_base()?;
        self.expect(Token::Semi)?;

        let source = self.extract_definition_source(&name);

        // Add const value to global_values for enum reference resolution
        self.global_values.insert(name.clone(), value);

        Ok(Const {
            name,
            value,
            base,
            source,
        })
    }

    fn parse_member(&mut self) -> Result<Member, ParseError> {
        // Record start position for source extraction (for anonymous unions)
        let type_start_byte = self.tokens.get(self.pos).map(|st| st.start).unwrap_or(0);

        // Track extracted definitions before parsing type (for fixing parent relationships)
        let extract_start_idx = self.extracted_definitions.len();

        let type_ = self.parse_type()?;

        // Record end position (after parsing type, before field name)
        let type_end_byte = if self.pos > 0 {
            self.tokens
                .get(self.pos - 1)
                .map(|st| st.end)
                .unwrap_or(self.source.len())
        } else {
            type_start_byte
        };

        let name = self.expect_ident()?;

        // Handle array suffix: name[size] or name<max>
        let type_ = self.parse_type_suffix(type_)?;

        // If this is an anonymous union, extract it as a separate definition
        let type_ = self.extract_anonymous_union_if_needed(
            type_,
            &name,
            type_start_byte,
            type_end_byte,
            extract_start_idx,
        );

        Ok(Member { name, type_ })
    }

    fn parse_union_arm(&mut self) -> Result<UnionArm, ParseError> {
        let mut cases = Vec::new();

        // Parse case(s) - multiple cases can share the same arm
        loop {
            match self.peek() {
                Token::Case => {
                    self.advance();
                    let value = match self.peek().clone() {
                        Token::Ident(name) => {
                            self.advance();
                            CaseValue::Ident(name)
                        }
                        Token::IntLiteral((value, _)) => {
                            self.advance();
                            CaseValue::Literal(value as i32)
                        }
                        other => {
                            return Err(self.unexpected_token_error("case value".to_string(), other))
                        }
                    };
                    self.expect(Token::Colon)?;
                    cases.push(UnionCase { value });
                }
                Token::Default => {
                    self.advance();
                    self.expect(Token::Colon)?;
                    // Default has no cases
                    break;
                }
                _ => break,
            }
        }

        // Parse the arm type
        let type_ = if *self.peek() == Token::Void {
            self.advance();
            self.expect(Token::Semi)?;
            None
        } else if *self.peek() == Token::Struct {
            // Inline struct in a union arm. XDR syntax:
            //   case FOO:  struct { int x; } fieldName;
            //
            // We need the field name *before* parsing the struct body so we can
            // set root_parent correctly (nested types inside the struct derive
            // their names from it). This requires a two-pass approach:
            //   Pass 1 (lookahead): skip the struct body by counting braces,
            //           read the field name and semicolon, record positions.
            //   Pass 2 (rewind):    rewind into the struct body and parse
            //           members with root_parent set to the generated name,
            //           then skip forward past the already-consumed tokens.
            Some(self.parse_inline_struct()?)
        } else {
            // Record start position for source extraction
            let type_start_byte = self.tokens.get(self.pos).map(|st| st.start).unwrap_or(0);

            // Track extracted definitions before parsing type (for fixing parent relationships)
            let extract_start_idx = self.extracted_definitions.len();

            let type_ = self.parse_type()?;

            // Record end position after parsing type
            let type_end_byte = if self.pos > 0 {
                self.tokens
                    .get(self.pos - 1)
                    .map(|st| st.end)
                    .unwrap_or(self.source.len())
            } else {
                type_start_byte
            };

            let field_name = self.expect_ident()?;
            let type_ = self.parse_type_suffix(type_)?;
            self.expect(Token::Semi)?;

            // If this is an anonymous union, extract it as a separate definition
            let type_ = self.extract_anonymous_union_if_needed(
                type_,
                &field_name,
                type_start_byte,
                type_end_byte,
                extract_start_idx,
            );

            Some(type_)
        };

        Ok(UnionArm { cases, type_ })
    }

    /// Parse an inline struct definition inside a union arm and extract it as a
    /// separate named type. Returns a `Type::Ident` referencing the extracted struct.
    ///
    /// Expects the parser to be positioned at the `struct` keyword.
    fn parse_inline_struct(&mut self) -> Result<Type, ParseError> {
        // Record position before 'struct' keyword for source extraction
        let source_start_byte = self.tokens.get(self.pos).map(|st| st.start).unwrap_or(0);

        self.advance(); // consume 'struct'
        self.expect(Token::LBrace)?;

        // --- Pass 1: lookahead to find the field name after the struct body ---
        let body_start_pos = self.pos;
        let mut brace_depth = 1;
        while brace_depth > 0 {
            match self.advance() {
                Token::LBrace => brace_depth += 1,
                Token::RBrace => brace_depth -= 1,
                Token::Eof => break,
                _ => {}
            }
        }

        let source_end_byte = if self.pos > 0 {
            self.tokens
                .get(self.pos - 1)
                .map(|st| st.end)
                .unwrap_or(self.source.len())
        } else {
            source_start_byte
        };

        let field_name = self.expect_ident()?;
        self.expect(Token::Semi)?;
        let after_semi_pos = self.pos;

        // --- Pass 2: rewind and parse the struct body properly ---
        self.pos = body_start_pos;

        let struct_name = if let Some(ref parent) = self.root_parent {
            generate_nested_type_name(parent, &field_name)
        } else {
            field_name.to_upper_camel_case()
        };

        let prev_root = self.root_parent.take();
        self.root_parent = Some(struct_name.clone());

        let mut members = Vec::new();
        while *self.peek() != Token::RBrace {
            let member = self.parse_member()?;
            members.push(member);
            self.expect(Token::Semi)?;
        }
        self.expect(Token::RBrace)?;

        self.root_parent = prev_root;

        // Advance past the field name and semicolon already consumed in pass 1
        self.pos = after_semi_pos;

        let source = if source_start_byte < source_end_byte && source_end_byte <= self.source.len()
        {
            self.source[source_start_byte..source_end_byte].to_string()
        } else {
            String::new()
        };

        let struct_def = Struct {
            name: struct_name.clone(),
            members,
            source,
            is_nested: true,
            parent: self.root_parent.clone(),
        };

        self.extracted_definitions
            .push(Definition::Struct(struct_def));

        Ok(Type::Ident(struct_name))
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.peek().clone() {
            Token::Int => {
                self.advance();
                Ok(Type::Int)
            }
            Token::Unsigned => {
                self.advance();
                match self.peek() {
                    Token::Int => {
                        self.advance();
                        Ok(Type::UnsignedInt)
                    }
                    Token::Hyper => {
                        self.advance();
                        Ok(Type::UnsignedHyper)
                    }
                    other => {
                        Err(self.unexpected_token_error("int or hyper".to_string(), other.clone()))
                    }
                }
            }
            Token::Hyper => {
                self.advance();
                Ok(Type::Hyper)
            }
            Token::Float => {
                self.advance();
                Ok(Type::Float)
            }
            Token::Double => {
                self.advance();
                Ok(Type::Double)
            }
            Token::Bool => {
                self.advance();
                Ok(Type::Bool)
            }
            Token::Opaque => {
                self.advance();
                self.parse_opaque_suffix()
            }
            Token::String => {
                self.advance();
                self.parse_string_suffix()
            }
            Token::Union => {
                // Anonymous union inside struct
                // union switch (type name) { ... }
                self.advance();
                self.expect(Token::Switch)?;
                self.expect(Token::LParen)?;
                let disc_type = self.parse_type()?;
                let disc_name = self.expect_ident()?;
                self.expect(Token::RParen)?;
                self.expect(Token::LBrace)?;

                let mut arms = Vec::new();
                let mut default_arm = None;

                while *self.peek() != Token::RBrace {
                    let arm = self.parse_union_arm()?;
                    if arm.cases.is_empty() {
                        default_arm = Some(Box::new(arm));
                    } else {
                        arms.push(arm);
                    }
                }
                self.expect(Token::RBrace)?;

                // Return an AnonymousUnion that will be extracted in parse_member
                Ok(Type::AnonymousUnion {
                    discriminant: Box::new(Discriminant {
                        name: disc_name,
                        type_: disc_type,
                    }),
                    arms,
                    default_arm,
                })
            }
            Token::Ident(name) => {
                self.advance();
                // Handle built-in type aliases
                let base_type = match name.as_str() {
                    "uint64" => Type::UnsignedHyper,
                    "int64" => Type::Hyper,
                    "uint32" => Type::UnsignedInt,
                    "int32" => Type::Int,
                    "TRUE" | "FALSE" => Type::Bool,
                    _ => Type::Ident(name),
                };
                // Check for optional type suffix (Type* field)
                if *self.peek() == Token::Star {
                    self.advance();
                    Ok(Type::Optional(Box::new(base_type)))
                } else {
                    Ok(base_type)
                }
            }
            other => Err(self.unexpected_token_error("type".to_string(), other)),
        }
    }

    fn parse_type_suffix(&mut self, base: Type) -> Result<Type, ParseError> {
        match self.peek() {
            Token::LBracket => {
                self.advance();
                let size = self.parse_size()?;
                self.expect(Token::RBracket)?;

                // Special case: opaque name[size] or string name[size]
                // means fixed opaque/string, not an array of opaque/string
                match base {
                    Type::OpaqueVar(None) => Ok(Type::OpaqueFixed(size)),
                    Type::String(None) => Ok(Type::OpaqueFixed(size)), // string with fixed size is opaque
                    _ => Ok(Type::Array {
                        element_type: Box::new(base),
                        size,
                    }),
                }
            }
            Token::LAngle => {
                self.advance();
                let max = if *self.peek() == Token::RAngle {
                    None
                } else {
                    Some(self.parse_size()?)
                };
                self.expect(Token::RAngle)?;

                // Special case: opaque name<max> or string name<max>
                // means variable opaque/string with max, not a var array
                match base {
                    Type::OpaqueVar(None) => Ok(Type::OpaqueVar(max)),
                    Type::String(None) => Ok(Type::String(max)),
                    _ => Ok(Type::VarArray {
                        element_type: Box::new(base),
                        max_size: max,
                    }),
                }
            }
            Token::Star => {
                // Optional: type *name
                self.advance();
                Ok(Type::Optional(Box::new(base)))
            }
            _ => Ok(base),
        }
    }

    fn parse_opaque_suffix(&mut self) -> Result<Type, ParseError> {
        match self.peek() {
            Token::LBracket => {
                // Fixed: opaque[size]
                self.advance();
                let size = self.parse_size()?;
                self.expect(Token::RBracket)?;
                Ok(Type::OpaqueFixed(size))
            }
            Token::LAngle => {
                // Variable: opaque<max> or opaque<>
                self.advance();
                let max = if *self.peek() == Token::RAngle {
                    None
                } else {
                    Some(self.parse_size()?)
                };
                self.expect(Token::RAngle)?;
                Ok(Type::OpaqueVar(max))
            }
            _ => {
                // Bare opaque - variable with no max (rare)
                Ok(Type::OpaqueVar(None))
            }
        }
    }

    fn parse_string_suffix(&mut self) -> Result<Type, ParseError> {
        match self.peek() {
            Token::LAngle => {
                self.advance();
                let max = if *self.peek() == Token::RAngle {
                    None
                } else {
                    Some(self.parse_size()?)
                };
                self.expect(Token::RAngle)?;
                Ok(Type::String(max))
            }
            _ => Ok(Type::String(None)),
        }
    }

    fn parse_size(&mut self) -> Result<Size, ParseError> {
        match self.peek().clone() {
            Token::IntLiteral((value, _)) => {
                self.advance();
                Ok(Size::Literal(value as u32))
            }
            Token::Ident(name) => {
                self.advance();
                Ok(Size::Named(name))
            }
            other => {
                Err(self.unexpected_token_error("size (integer or identifier)".to_string(), other))
            }
        }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .map(|st| &st.token)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> &Token {
        let token = self
            .tokens
            .get(self.pos)
            .map(|st| &st.token)
            .unwrap_or(&Token::Eof);
        self.pos += 1;
        token
    }

    /// Compute the (line, column) for the current token position, both 1-based.
    fn current_position(&self) -> (usize, usize) {
        let byte_offset = self
            .tokens
            .get(self.pos.saturating_sub(1))
            .map(|st| st.start)
            .unwrap_or(self.source.len());
        let prefix = &self.source[..byte_offset.min(self.source.len())];
        let line = prefix.chars().filter(|&c| c == '\n').count() + 1;
        let col = match prefix.rfind('\n') {
            Some(nl) => byte_offset - nl,
            None => byte_offset + 1,
        };
        (line, col)
    }

    /// Create an `UnexpectedToken` error with the current position.
    fn unexpected_token_error(&self, expected: String, got: Token) -> ParseError {
        let (line, col) = self.current_position();
        ParseError::UnexpectedToken {
            expected,
            got,
            line,
            col,
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let token = self.advance().clone();
        if token == expected {
            Ok(())
        } else {
            Err(self.unexpected_token_error(format!("{expected:?}"), token))
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        let token = self.advance().clone();
        match token {
            Token::Ident(s) => Ok(s),
            _ => Err(self.unexpected_token_error("identifier".to_string(), token)),
        }
    }

    /// Parse an integer literal, returning both the value and whether it was in hex format.
    fn expect_int_with_base(&mut self) -> Result<(i64, IntBase), ParseError> {
        let token = self.advance().clone();
        match token {
            Token::IntLiteral((value, base)) => Ok((value, base)),
            _ => Err(self.unexpected_token_error("integer literal".to_string(), token)),
        }
    }

    /// If `type_` is an anonymous union, extract it as a named union definition and return
    /// a `Type::Ident` referencing the extracted type. Otherwise return `type_` unchanged.
    ///
    /// `field_name` is the field that holds the union (used for name generation).
    /// `type_start_byte`/`type_end_byte` delimit the source text for the anonymous union.
    /// `extract_start_idx` is the index into `self.extracted_definitions` from before
    /// the type was parsed, used to fix parent relationships on definitions extracted
    /// during the union's parsing.
    fn extract_anonymous_union_if_needed(
        &mut self,
        type_: Type,
        field_name: &str,
        type_start_byte: usize,
        type_end_byte: usize,
        extract_start_idx: usize,
    ) -> Type {
        match type_ {
            Type::AnonymousUnion {
                discriminant,
                arms,
                default_arm,
            } => {
                // Generate the name: root_parent + field_name
                let union_name = if let Some(ref parent) = self.root_parent {
                    generate_nested_type_name(parent, field_name)
                } else {
                    generate_nested_type_name(field_name, "Union")
                };

                // Extract source text for the anonymous union
                let source =
                    if type_start_byte < type_end_byte && type_end_byte <= self.source.len() {
                        self.source[type_start_byte..type_end_byte].to_string()
                    } else {
                        String::new()
                    };

                // Create the Union definition (unbox the discriminant)
                let union_def = Union {
                    name: union_name.clone(),
                    discriminant: *discriminant,
                    arms,
                    default_arm,
                    source,
                    is_nested: true,
                    parent: self.root_parent.clone(),
                };

                // Fix up parent relationships for any definitions extracted during union parsing
                // (e.g., inline structs inside union arms should have this union as their parent)
                // Only update if current parent is the root_parent (not already a more specific parent)
                for def in &mut self.extracted_definitions[extract_start_idx..] {
                    match def {
                        Definition::Struct(s) if s.is_nested && s.parent == self.root_parent => {
                            s.parent = Some(union_name.clone());
                        }
                        Definition::Union(u) if u.is_nested && u.parent == self.root_parent => {
                            u.parent = Some(union_name.clone());
                        }
                        _ => {}
                    }
                }

                // Add to extracted definitions
                self.extracted_definitions
                    .push(Definition::Union(union_def));

                // Return a reference to the extracted type
                Type::Ident(union_name)
            }
            other => other,
        }
    }

    /// Resolve an enum value reference, searching the current enum members
    /// and then previously parsed enums/consts.
    fn resolve_enum_value(&self, name: &str, members: &[EnumMember]) -> Result<i32, ParseError> {
        // First check if it's in the current enum being parsed
        for m in members {
            if m.name == name {
                return Ok(m.value);
            }
        }
        // Check global values (previously parsed enums and consts)
        if let Some(&value) = self.global_values.get(name) {
            return Ok(value as i32);
        }
        let (line, col) = self.current_position();
        Err(ParseError::UnresolvedEnumValue {
            name: name.to_string(),
            line,
            col,
        })
    }

    /// Extract the source text for a definition using the tracked start position.
    fn extract_definition_source(&self, _name: &str) -> String {
        // Get the byte range from the start token to the current token
        let start_byte = self
            .tokens
            .get(self.def_start_pos)
            .map(|st| st.start)
            .unwrap_or(0);

        // The end is the current position's end (previous token's end after parsing)
        let end_byte = if self.pos > 0 {
            self.tokens
                .get(self.pos - 1)
                .map(|st| st.end)
                .unwrap_or(self.source.len())
        } else {
            start_byte
        };

        // Extract and return the source text
        if start_byte < end_byte && end_byte <= self.source.len() {
            self.source[start_byte..end_byte].to_string()
        } else {
            String::new()
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("lexer error: {0}")]
    Lex(#[from] LexError),
    #[error("{line}:{col}: unexpected token: expected {expected}, got {got:?}")]
    UnexpectedToken {
        expected: String,
        got: Token,
        line: usize,
        col: usize,
    },
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("{line}:{col}: unresolved enum value reference: {name}")]
    UnresolvedEnumValue {
        name: String,
        line: usize,
        col: usize,
    },
}

/// Fix parent relationships for nested types based on naming patterns.
///
/// During parsing, each extracted nested type is assigned its immediate parent.
/// However, when inline structs inside union arms themselves contain anonymous
/// unions, the deeply nested union may only know about the top-level parent.
///
/// For example, given XDR like:
/// ```text
/// union Outer switch (int v) {
///     case 0:
///         struct {
///             union switch (int w) { case 0: void; } innerField;
///         } outerField;
/// };
/// ```
/// Parsing produces three types:
///   - `OuterOuterFieldInnerField` (the anonymous union) — initially parent = `Outer`
///   - `OuterOuterField` (the inline struct) — parent = `Outer`
///   - `Outer` (the top-level union)
///
/// This function detects that `OuterOuterFieldInnerField` starts with
/// `OuterOuterField` and reassigns its parent from `Outer` to `OuterOuterField`.
fn fix_parent_relationships(definitions: &mut [Definition]) {
    // Collect all nested type names
    let nested_names: Vec<String> = definitions
        .iter()
        .filter_map(|def| {
            if def.is_nested() {
                Some(def.name().to_string())
            } else {
                None
            }
        })
        .collect();

    // For each nested type, find if there's a "better" parent
    // (a nested type whose name is a prefix and longer than current parent)
    for def in definitions.iter_mut() {
        if !def.is_nested() {
            continue;
        }
        let name = def.name().to_string();
        let current_parent = def.parent().map(|s| s.to_string());

        // Find the longest prefix match among nested types (excluding self)
        let best_parent = nested_names
            .iter()
            .filter(|&candidate| {
                candidate != &name
                    && name.starts_with(candidate)
                    && current_parent
                        .as_ref()
                        .map(|p| candidate.len() > p.len())
                        .unwrap_or(true)
            })
            .max_by_key(|s| s.len());

        if let Some(new_parent) = best_parent {
            match def {
                Definition::Struct(s) => s.parent = Some(new_parent.clone()),
                Definition::Union(u) => u.parent = Some(new_parent.clone()),
                _ => {}
            }
        }
    }
}

/// Generate a nested type name from parent and field name.
fn generate_nested_type_name(parent: &str, field: &str) -> String {
    format!("{}{}", parent, field.to_upper_camel_case())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_struct() {
        let input = "struct Foo { int x; unsigned hyper y; };";
        let mut parser = Parser::new(input).unwrap();
        let spec = parser.parse().unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Struct(s) = &spec.definitions[0] {
            assert_eq!(s.name, "Foo");
            assert_eq!(s.members.len(), 2);
            assert_eq!(s.members[0].name, "x");
            assert_eq!(s.members[0].type_, Type::Int);
            assert_eq!(s.members[1].name, "y");
            assert_eq!(s.members[1].type_, Type::UnsignedHyper);
        } else {
            panic!("Expected struct");
        }
    }

    #[test]
    fn test_parse_enum() {
        let input = "enum Color { RED = 0, GREEN = 1, BLUE = 2 };";
        let mut parser = Parser::new(input).unwrap();
        let spec = parser.parse().unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Enum(e) = &spec.definitions[0] {
            assert_eq!(e.name, "Color");
            assert_eq!(e.members.len(), 3);
            assert_eq!(e.members[0].name, "RED");
            assert_eq!(e.members[0].value, 0);
        } else {
            panic!("Expected enum");
        }
    }

    #[test]
    fn test_parse_typedef() {
        let input = "typedef opaque Hash[32];";
        let mut parser = Parser::new(input).unwrap();
        let spec = parser.parse().unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Typedef(t) = &spec.definitions[0] {
            assert_eq!(t.name, "Hash");
            assert_eq!(t.type_, Type::OpaqueFixed(Size::Literal(32)));
        } else {
            panic!("Expected typedef");
        }
    }

    #[test]
    fn test_parse_namespace() {
        let input = "namespace stellar { struct Foo { int x; }; };";
        let mut parser = Parser::new(input).unwrap();
        let spec = parser.parse().unwrap();

        assert_eq!(spec.namespaces.len(), 1);
        assert_eq!(spec.namespaces[0].name, "stellar");
        assert_eq!(spec.namespaces[0].definitions.len(), 1);
    }

    #[test]
    fn test_fix_parent_relationships() {
        // Simulate the scenario: an inline struct inside a union arm contains
        // an anonymous union. The deeply nested union should have its parent
        // reassigned from the top-level union to the inline struct.
        let mut definitions = vec![
            Definition::Union(Union {
                name: "OuterOuterFieldInnerField".to_string(),
                discriminant: Discriminant {
                    name: "w".to_string(),
                    type_: Type::Int,
                },
                arms: vec![],
                default_arm: None,
                source: String::new(),
                is_nested: true,
                parent: Some("Outer".to_string()),
            }),
            Definition::Struct(Struct {
                name: "OuterOuterField".to_string(),
                members: vec![],
                source: String::new(),
                is_nested: true,
                parent: Some("Outer".to_string()),
            }),
            Definition::Union(Union {
                name: "Outer".to_string(),
                discriminant: Discriminant {
                    name: "v".to_string(),
                    type_: Type::Int,
                },
                arms: vec![],
                default_arm: None,
                source: String::new(),
                is_nested: false,
                parent: None,
            }),
        ];

        fix_parent_relationships(&mut definitions);

        // The deeply nested union should now point to the inline struct
        assert_eq!(
            definitions[0].parent(),
            Some("OuterOuterField"),
            "deeply nested type should have its parent reassigned to the inline struct"
        );
        // The inline struct's parent should remain unchanged
        assert_eq!(definitions[1].parent(), Some("Outer"));
        // The top-level union has no parent
        assert_eq!(definitions[2].parent(), None);
    }
}
