//! XDR parser (recursive descent).

use std::collections::HashMap;

use crate::ast::*;
use crate::lexer::{IntBase, LexError, Lexer, SpannedToken, Token};
use heck::ToUpperCamelCase;
use sha2::{Digest as _, Sha256};
use thiserror::Error;

/// Parse multiple XDR source files into a single AST.
///
/// Each entry is a `(file_name, source_text)` pair. Files are parsed in order,
/// with definitions from earlier files visible to later files (e.g. enum values
/// and consts can be referenced across files). Each definition tracks which file
/// it came from via `file_index`.
///
/// File SHA256 hashes are computed and stored in `XdrSpec::files`.
pub fn parse_files(files: &[(&str, &str)]) -> Result<XdrSpec, ParseError> {
    let mut spec = XdrSpec::default();
    let mut global_values = HashMap::new();

    for (file_index, (name, content)) in files.iter().enumerate() {
        let hash = format!("{:x}", Sha256::digest(content.as_bytes()));
        spec.files.push(XdrFile {
            name: name.to_string(),
            sha256: hash,
        });

        let mut parser = Parser::new(content, global_values)?;
        parser.file_index = file_index;
        let file_spec = parser.parse()?;
        global_values = parser.global_values;

        spec.definitions.extend(file_spec.definitions);
        for mut ns in file_spec.namespaces {
            // Set file_index on namespace definitions too
            for def in &mut ns.definitions {
                set_file_index(def, file_index);
            }
            spec.namespaces.push(ns);
        }
    }

    Ok(spec)
}

/// Parse a single XDR source string into an AST.
///
/// Convenience wrapper around `parse_files` for single-file use.
pub fn parse(source: &str) -> Result<XdrSpec, ParseError> {
    parse_files(&[("", source)])
}

/// Set the file_index on a definition.
fn set_file_index(def: &mut Definition, index: usize) {
    match def {
        Definition::Struct(s) => s.file_index = index,
        Definition::Enum(e) => e.file_index = index,
        Definition::Union(u) => u.file_index = index,
        Definition::Typedef(t) => t.file_index = index,
        Definition::Const(c) => c.file_index = index,
    }
}

struct Parser {
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
    global_values: HashMap<String, i64>,
    /// Index of the file currently being parsed
    file_index: usize,
    /// Stack of cfg conditions from enclosing `#ifdef`/`#else` blocks.
    cfg_stack: Vec<CfgExpr>,
    /// Stack tracking the initial condition for each active inline `#ifdef`
    /// block (inside enum/union bodies), so that `parse_ifdef_branch` can
    /// compute the `#else` condition and `parse_ifdef_exit` can clean up.
    ifdef_seen_stack: Vec<CfgExpr>,
}

impl Parser {
    fn new(source: &str, global_values: HashMap<String, i64>) -> Result<Self, ParseError> {
        let lexer = Lexer::new(source);
        let (tokens, source) = lexer.tokenize_with_spans()?;
        Ok(Self {
            tokens,
            pos: 0,
            source,
            def_start_pos: 0,
            extracted_definitions: Vec::new(),
            root_parent: None,
            global_values,
            file_index: 0,
            cfg_stack: Vec::new(),
            ifdef_seen_stack: Vec::new(),
        })
    }

    /// Parse the entire input.
    fn parse(&mut self) -> Result<XdrSpec, ParseError> {
        let mut spec = XdrSpec::default();

        self.parse_definitions_until(&mut spec.definitions, &mut spec.namespaces, |tok| {
            *tok == Token::Eof
        })?;

        // Any remaining extracted definitions (shouldn't be any, but just in case)
        for extracted in self.extracted_definitions.drain(..) {
            spec.definitions.push(extracted);
        }

        Ok(spec)
    }

    /// Parse definitions until `stop` returns true.
    /// Handles `#ifdef`/`#else`/`#endif` blocks.
    fn parse_definitions_until(
        &mut self,
        definitions: &mut Vec<Definition>,
        namespaces: &mut Vec<Namespace>,
        stop: impl Fn(&Token) -> bool,
    ) -> Result<(), ParseError> {
        while !stop(self.peek()) {
            // Skip any extra semicolons
            while *self.peek() == Token::Semi {
                self.advance();
            }
            if stop(self.peek()) {
                break;
            }
            match self.peek().clone() {
                Token::Namespace => {
                    let ns = self.parse_namespace()?;
                    namespaces.push(ns);
                }
                Token::IfDef(_) => {
                    self.parse_ifdef_block(definitions, namespaces)?;
                }
                Token::Else | Token::EndIf => {
                    return Err(self.make_unexpected_directive_error());
                }
                _ => {
                    self.parse_definition_into(definitions)?;
                }
            }
        }
        Ok(())
    }

    /// Parse an `#ifdef` block including `#else`/`#endif`.
    fn parse_ifdef_block(
        &mut self,
        definitions: &mut Vec<Definition>,
        namespaces: &mut Vec<Namespace>,
    ) -> Result<(), ParseError> {
        // Parse the initial #ifdef
        let first_cfg = match self.peek().clone() {
            Token::IfDef(name) => {
                self.advance();
                CfgExpr::Feature(name)
            }
            _ => unreachable!(),
        };

        // Parse the #ifdef body
        self.cfg_stack.push(first_cfg.clone());
        self.parse_definitions_until(definitions, namespaces, |tok| {
            matches!(tok, Token::Else | Token::EndIf | Token::Eof)
        })?;
        self.cfg_stack.pop();

        // Handle #else
        if *self.peek() == Token::Else {
            self.advance();

            let else_cfg = first_cfg.negate();

            self.cfg_stack.push(else_cfg);
            self.parse_definitions_until(definitions, namespaces, |tok| {
                matches!(tok, Token::EndIf | Token::Eof)
            })?;
            self.cfg_stack.pop();
        }

        // Expect #endif
        if *self.peek() == Token::EndIf {
            self.advance();
        } else {
            return Err(self.unexpected_token_error("#endif".to_string(), self.peek().clone()));
        }

        Ok(())
    }

    /// Enter an `#ifdef` block inline (inside an enum or union body).
    /// Pushes the initial condition onto `cfg_stack` and records it in
    /// `ifdef_seen_stack` so that `parse_ifdef_branch` and `parse_ifdef_exit`
    /// can manage `#else`/`#endif` later.
    fn parse_ifdef_enter(&mut self) -> Result<(), ParseError> {
        let first_cfg = match self.peek().clone() {
            Token::IfDef(name) => {
                self.advance();
                CfgExpr::Feature(name)
            }
            _ => unreachable!(),
        };
        self.ifdef_seen_stack.push(first_cfg.clone());
        self.cfg_stack.push(first_cfg);
        Ok(())
    }

    /// Handle an `#else` token inside an inline ifdef block.
    /// Pops the current branch's cfg and pushes the negated cfg.
    fn parse_ifdef_branch(&mut self) -> Result<(), ParseError> {
        if self.ifdef_seen_stack.is_empty() {
            return Err(self.make_unexpected_directive_error());
        }
        self.cfg_stack.pop();
        self.advance(); // consume #else
        let else_cfg = self.ifdef_seen_stack.last().unwrap().clone().negate();
        self.cfg_stack.push(else_cfg);
        Ok(())
    }

    /// Handle an `#endif` token inside an inline ifdef block.
    /// Pops the current branch's cfg and the seen_conditions entry.
    fn parse_ifdef_exit(&mut self) -> Result<(), ParseError> {
        if self.ifdef_seen_stack.is_empty() {
            return Err(self.make_unexpected_directive_error());
        }
        self.cfg_stack.pop();
        self.ifdef_seen_stack.pop();
        self.advance(); // consume EndIf
        Ok(())
    }

    /// Compute the current cfg expression from the cfg_stack.
    fn current_cfg(&self) -> Option<CfgExpr> {
        match self.cfg_stack.len() {
            0 => None,
            1 => Some(self.cfg_stack[0].clone()),
            _ => {
                // Flatten nested All() expressions for cleaner output.
                let mut parts = Vec::new();
                for expr in &self.cfg_stack {
                    match expr {
                        CfgExpr::All(inner) => parts.extend(inner.iter().cloned()),
                        other => parts.push(other.clone()),
                    }
                }
                Some(CfgExpr::All(parts))
            }
        }
    }

    fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
        self.expect(Token::Namespace)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;

        let mut definitions = Vec::new();
        let mut namespaces = Vec::new();
        self.parse_definitions_until(&mut definitions, &mut namespaces, |tok| {
            matches!(tok, Token::RBrace | Token::Eof)
        })?;

        self.expect(Token::RBrace)?;

        Ok(Namespace {
            name,
            definitions,
            namespaces,
        })
    }

    /// Parse a single definition, prepending any extracted nested definitions
    /// (anonymous unions, inline structs) so they appear just before their parent.
    fn parse_definition_into(&mut self, out: &mut Vec<Definition>) -> Result<(), ParseError> {
        let extract_start = self.extracted_definitions.len();

        let def = self.parse_definition(extract_start)?;

        // Insert any newly extracted definitions before this definition
        for extracted in self.extracted_definitions.drain(extract_start..) {
            out.push(extracted);
        }

        out.push(def);
        Ok(())
    }

    fn parse_definition(&mut self, extract_start: usize) -> Result<Definition, ParseError> {
        // Mark the start of this definition for source extraction
        self.def_start_pos = self.pos;

        let cfg = self.current_cfg();

        let mut def = match self.peek() {
            Token::Struct => self.parse_struct().map(Definition::Struct),
            Token::Enum => self.parse_enum().map(Definition::Enum),
            Token::Union => self.parse_union().map(Definition::Union),
            Token::Typedef => self.parse_typedef().map(Definition::Typedef),
            Token::Const => self.parse_const().map(Definition::Const),
            other => Err(self.unexpected_token_error(
                "struct, enum, union, typedef, or const".to_string(),
                other.clone(),
            )),
        }?;

        def.set_cfg(cfg.clone());

        // Also set cfg on any extracted nested definitions from this definition.
        // Only apply to definitions extracted during this parse (from extract_start).
        if cfg.is_some() {
            for extracted in &mut self.extracted_definitions[extract_start..] {
                if extracted.cfg().is_none() {
                    extracted.set_cfg(cfg.clone());
                }
            }
        }

        Ok(def)
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
        let source = self.extract_definition_source();

        Ok(Struct {
            name,
            members,
            source,
            is_nested: false,
            parent: None,
            file_index: self.file_index,
            cfg: None,
        })
    }

    fn parse_enum(&mut self) -> Result<Enum, ParseError> {
        self.expect(Token::Enum)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;

        let ifdef_depth_before = self.ifdef_seen_stack.len();
        let mut members: Vec<(String, i32, Option<CfgExpr>)> = Vec::new();
        loop {
            // Handle #ifdef/#else/#endif inside enum body
            match self.peek().clone() {
                Token::IfDef(_) => {
                    self.parse_ifdef_enter()?;
                    continue;
                }
                Token::Else => {
                    self.parse_ifdef_branch()?;
                    continue;
                }
                Token::EndIf => {
                    self.parse_ifdef_exit()?;
                    continue;
                }
                Token::RBrace => break,
                _ => {}
            }

            let member_name = self.expect_ident()?;
            self.expect(Token::Eq)?;

            // Value can be integer or identifier (reference to another enum value)
            let value = match self.peek().clone() {
                Token::IntLiteral((value, _)) => {
                    self.advance();
                    self.try_i64_to_i32(value)?
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

            members.push((member_name, value, self.current_cfg()));

            match self.peek() {
                Token::Comma => {
                    self.advance();
                    // After comma, skip to } or next member (could be #ifdef or #endif)
                    if *self.peek() == Token::RBrace {
                        break;
                    }
                }
                Token::RBrace => break,
                // Allow #else/#endif directly after a member without a comma
                Token::EndIf | Token::Else => {}
                other => {
                    return Err(self.unexpected_token_error(", or }".to_string(), other.clone()))
                }
            }
        }

        self.expect(Token::RBrace)?;

        if self.ifdef_seen_stack.len() != ifdef_depth_before {
            return Err(self.unexpected_token_error("#endif".to_string(), Token::RBrace));
        }

        self.expect(Token::Semi)?;

        let source = self.extract_definition_source();

        // Add all members to global_values for cross-enum resolution
        for (name, value, _) in &members {
            self.global_values.insert(name.clone(), *value as i64);
        }

        let mut e = Enum::new(name, members, source);
        e.file_index = self.file_index;
        Ok(e)
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

        let arms = self.parse_union_body()?;

        // Restore previous root_parent
        self.root_parent = prev_root;

        self.expect(Token::RBrace)?;
        self.expect(Token::Semi)?;

        let source = self.extract_definition_source();

        Ok(Union {
            name,
            discriminant: UnionDiscriminant {
                name: disc_name,
                type_: disc_type,
            },
            arms,
            source,
            is_nested: false,
            parent: None,
            file_index: self.file_index,
            cfg: None,
        })
    }

    fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
        self.expect(Token::Typedef)?;

        let type_ = self.parse_type()?;
        let name = self.expect_ident()?;

        // Handle array suffix on typedef name
        let type_ = self.parse_type_suffix(type_)?;

        self.expect(Token::Semi)?;

        let source = self.extract_definition_source();

        Ok(Typedef {
            name,
            type_,
            source,
            file_index: self.file_index,
            cfg: None,
        })
    }

    fn parse_const(&mut self) -> Result<Const, ParseError> {
        self.expect(Token::Const)?;
        let name = self.expect_ident()?;
        self.expect(Token::Eq)?;
        let (value, base) = self.expect_int_with_base()?;
        self.expect(Token::Semi)?;

        let source = self.extract_definition_source();

        // Add const value to global_values for enum reference resolution
        self.global_values.insert(name.clone(), value);

        Ok(Const {
            name,
            value,
            base,
            source,
            file_index: self.file_index,
            cfg: None,
        })
    }

    fn parse_member(&mut self) -> Result<StructMember, ParseError> {
        let ctx = self.type_parse_context();
        let parsed = self.parse_type_or_anon_union()?;
        let type_end_byte = self.prev_end_byte();

        let name = self.expect_ident()?;

        let type_ = match parsed {
            ParsedType::AnonymousUnion { discriminant, arms } => {
                self.extract_anonymous_union(discriminant, arms, &name, &ctx, type_end_byte)
            }
            ParsedType::Type(t) => self.parse_type_suffix(t)?,
        };

        Ok(StructMember { name, type_ })
    }

    /// Parse the body of a union (the arms inside the braces).
    /// Default arms are not supported — the generated code always emits a
    /// catch-all `_ => Err(Error::Invalid)`. If a default arm is encountered,
    /// a parse error is returned.
    fn parse_union_body(&mut self) -> Result<Vec<UnionArm>, ParseError> {
        let ifdef_depth_before = self.ifdef_seen_stack.len();
        let mut arms = Vec::new();

        while *self.peek() != Token::RBrace {
            // Handle #ifdef/#else/#endif inside union body
            match self.peek().clone() {
                Token::IfDef(_) => {
                    self.parse_ifdef_enter()?;
                    continue;
                }
                Token::Else => {
                    self.parse_ifdef_branch()?;
                    continue;
                }
                Token::EndIf => {
                    self.parse_ifdef_exit()?;
                    continue;
                }
                _ => {}
            }

            let cfg = self.current_cfg();
            let mut arm = self.parse_union_arm()?;
            arm.cfg = cfg;
            if arm.cases.is_empty() {
                let (line, col) = self.current_position();
                return Err(ParseError::UnsupportedDefaultArm { line, col });
            }
            arms.push(arm);
        }

        if self.ifdef_seen_stack.len() != ifdef_depth_before {
            return Err(self.unexpected_token_error("#endif".to_string(), Token::RBrace));
        }

        Ok(arms)
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
                            UnionCaseValue::Ident(name)
                        }
                        Token::IntLiteral((value, _)) => {
                            self.advance();
                            UnionCaseValue::Literal(self.try_i64_to_i32(value)?)
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

        // Parse the arm type and field name
        let (type_, arm_name) = if *self.peek() == Token::Void {
            self.advance();
            self.expect(Token::Semi)?;
            (None, String::new())
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
            let (t, field_name) = self.parse_inline_struct()?;
            (Some(t), field_name)
        } else {
            let ctx = self.type_parse_context();
            let parsed = self.parse_type_or_anon_union()?;
            let type_end_byte = self.prev_end_byte();

            let field_name = self.expect_ident()?;

            let type_ = match parsed {
                ParsedType::AnonymousUnion { discriminant, arms } => self.extract_anonymous_union(
                    discriminant,
                    arms,
                    &field_name,
                    &ctx,
                    type_end_byte,
                ),
                ParsedType::Type(t) => self.parse_type_suffix(t)?,
            };
            self.expect(Token::Semi)?;

            (Some(type_), field_name)
        };

        Ok(UnionArm {
            cases,
            name: arm_name,
            type_,
            cfg: None,
        })
    }

    /// Parse an inline struct definition inside a union arm and extract it as a
    /// separate named type. Returns `(Type::Ident, field_name)`.
    ///
    /// Expects the parser to be positioned at the `struct` keyword.
    fn parse_inline_struct(&mut self) -> Result<(Type, String), ParseError> {
        // Record position before 'struct' keyword for source extraction
        let source_start_byte = self.current_start_byte();

        self.advance(); // consume 'struct'
        self.expect(Token::LBrace)?;

        // --- Pass 1: lookahead to find the field name after the struct body ---
        let body_start_pos = self.pos;
        let mut brace_depth = 1;
        while brace_depth > 0 {
            match self.advance() {
                Token::LBrace => brace_depth += 1,
                Token::RBrace => brace_depth -= 1,
                Token::Eof => return Err(ParseError::UnexpectedEof),
                _ => {}
            }
        }

        let source_end_byte = self.prev_end_byte();

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

        let source = self.source_slice(source_start_byte, source_end_byte);

        let struct_def = Struct {
            name: struct_name.clone(),
            members,
            source,
            is_nested: true,
            parent: self.root_parent.clone(),
            file_index: self.file_index,
            cfg: None,
        };

        self.extracted_definitions
            .push(Definition::Struct(struct_def));

        Ok((Type::Ident(struct_name), field_name))
    }

    /// Parse a type that must be a regular type (not an anonymous union).
    /// Used for discriminants, typedefs, and other contexts where inline
    /// unions cannot appear.
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.parse_type_or_anon_union()? {
            ParsedType::Type(t) => Ok(t),
            ParsedType::AnonymousUnion { .. } => {
                Err(self.unexpected_token_error("type".to_string(), Token::Union))
            }
        }
    }

    /// Parse a type expression, which may be an anonymous union.
    /// Callers must handle the `ParsedType::AnonymousUnion` case by
    /// extracting it into a named definition.
    fn parse_type_or_anon_union(&mut self) -> Result<ParsedType, ParseError> {
        match self.peek().clone() {
            Token::Int => {
                self.advance();
                Ok(ParsedType::Type(Type::Int))
            }
            Token::Unsigned => {
                self.advance();
                match self.peek() {
                    Token::Int => {
                        self.advance();
                        Ok(ParsedType::Type(Type::UnsignedInt))
                    }
                    Token::Hyper => {
                        self.advance();
                        Ok(ParsedType::Type(Type::UnsignedHyper))
                    }
                    other => {
                        Err(self.unexpected_token_error("int or hyper".to_string(), other.clone()))
                    }
                }
            }
            Token::Hyper => {
                self.advance();
                Ok(ParsedType::Type(Type::Hyper))
            }
            Token::Float => {
                self.advance();
                Ok(ParsedType::Type(Type::Float))
            }
            Token::Double => {
                self.advance();
                Ok(ParsedType::Type(Type::Double))
            }
            Token::Bool => {
                self.advance();
                Ok(ParsedType::Type(Type::Bool))
            }
            Token::Opaque => {
                self.advance();
                self.parse_opaque_suffix().map(ParsedType::Type)
            }
            Token::String => {
                self.advance();
                self.parse_string_suffix().map(ParsedType::Type)
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

                let arms = self.parse_union_body()?;
                self.expect(Token::RBrace)?;

                Ok(ParsedType::AnonymousUnion {
                    discriminant: UnionDiscriminant {
                        name: disc_name,
                        type_: disc_type,
                    },
                    arms,
                })
            }
            Token::Ident(name) => {
                self.advance();
                // Handle built-in type aliases
                let base_type = match name.as_str() {
                    "TRUE" | "FALSE" => Type::Bool,
                    _ => Type::Ident(name),
                };
                // Check for optional type suffix (Type* field)
                if *self.peek() == Token::Star {
                    self.advance();
                    Ok(ParsedType::Type(Type::Optional(Box::new(base_type))))
                } else {
                    Ok(ParsedType::Type(base_type))
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
                Ok(Size::Literal(self.try_i64_to_u32(value)?))
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

    /// Get the byte offset where the current token starts.
    fn current_start_byte(&self) -> usize {
        self.tokens.get(self.pos).map(|st| st.start).unwrap_or(0)
    }

    /// Get the byte offset where the previous token ends.
    fn prev_end_byte(&self) -> usize {
        if self.pos > 0 {
            self.tokens
                .get(self.pos - 1)
                .map(|st| st.end)
                .unwrap_or(self.source.len())
        } else {
            0
        }
    }

    /// Extract a source slice between two byte offsets.
    fn source_slice(&self, start: usize, end: usize) -> String {
        debug_assert!(
            start < end && end <= self.source.len(),
            "source_slice out of bounds: start={start}, end={end}, len={}",
            self.source.len()
        );
        if start < end && end <= self.source.len() {
            self.source[start..end].to_string()
        } else {
            String::new()
        }
    }

    /// Compute the (line, column) for the current token position, both 1-based.
    fn peek_position(&self) -> (usize, usize) {
        let byte_offset = self
            .tokens
            .get(self.pos)
            .map(|st| st.start)
            .unwrap_or(self.source.len());
        self.position_from_byte_offset(byte_offset)
    }

    fn current_position(&self) -> (usize, usize) {
        let byte_offset = self
            .tokens
            .get(self.pos.saturating_sub(1))
            .map(|st| st.start)
            .unwrap_or(self.source.len());
        self.position_from_byte_offset(byte_offset)
    }

    fn position_from_byte_offset(&self, byte_offset: usize) -> (usize, usize) {
        let prefix = &self.source[..byte_offset.min(self.source.len())];
        let line = prefix.chars().filter(|&c| c == '\n').count() + 1;
        let col = match prefix.rfind('\n') {
            Some(nl) => byte_offset - nl,
            None => byte_offset + 1,
        };
        (line, col)
    }

    /// Try to convert an i64 to the target integer type, returning an error with position on overflow.
    fn try_i64_to_i32(&self, value: i64) -> Result<i32, ParseError> {
        i32::try_from(value).map_err(|_| {
            let (line, col) = self.current_position();
            ParseError::IntegerOverflow { value, line, col }
        })
    }

    fn try_i64_to_u32(&self, value: i64) -> Result<u32, ParseError> {
        u32::try_from(value).map_err(|_| {
            let (line, col) = self.current_position();
            ParseError::IntegerOverflow { value, line, col }
        })
    }

    /// Create an `UnexpectedDirective` error for the current (not-yet-consumed) token.
    fn make_unexpected_directive_error(&self) -> ParseError {
        let directive = match self.peek() {
            Token::Else => "else",
            Token::EndIf => "endif",
            _ => "unknown",
        };
        let (line, col) = self.peek_position();
        ParseError::UnexpectedDirective {
            directive: directive.to_string(),
            line,
            col,
        }
    }

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

    /// Snapshot the parser state needed before parsing a type expression, so
    /// that anonymous unions can later be extracted with correct source spans
    /// and parent fixups.
    fn type_parse_context(&self) -> TypeParseContext {
        TypeParseContext {
            start_byte: self.current_start_byte(),
            extract_start_idx: self.extracted_definitions.len(),
        }
    }

    /// Extract an anonymous union as a named `Union` definition and return
    /// a `Type::Ident` referencing it.
    ///
    /// `field_name` is the field that holds the union (used for name generation).
    /// `ctx` captures the parser state from before the type was parsed.
    /// `type_end_byte` is the byte offset where the type expression ended.
    fn extract_anonymous_union(
        &mut self,
        discriminant: UnionDiscriminant,
        arms: Vec<UnionArm>,
        field_name: &str,
        ctx: &TypeParseContext,
        type_end_byte: usize,
    ) -> Type {
        // Generate the name: root_parent + field_name
        let union_name = if let Some(ref parent) = self.root_parent {
            generate_nested_type_name(parent, field_name)
        } else {
            generate_nested_type_name(field_name, "Union")
        };

        // Extract source text for the anonymous union
        let source = self.source_slice(ctx.start_byte, type_end_byte);

        let union_def = Union {
            name: union_name.clone(),
            discriminant,
            arms,
            source,
            is_nested: true,
            parent: self.root_parent.clone(),
            file_index: self.file_index,
            cfg: None,
        };

        // Fix up parent relationships for any definitions extracted during union parsing
        // (e.g., inline structs inside union arms should have this union as their parent)
        // Only update if current parent is the root_parent (not already a more specific parent)
        for def in &mut self.extracted_definitions[ctx.extract_start_idx..] {
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

    /// Resolve an enum value reference, searching the current enum members
    /// and then previously parsed enums/consts.
    fn resolve_enum_value(
        &self,
        name: &str,
        members: &[(String, i32, Option<CfgExpr>)],
    ) -> Result<i32, ParseError> {
        // First check if it's in the current enum being parsed
        for (member_name, value, _) in members {
            if member_name == name {
                return Ok(*value);
            }
        }
        // Check global values (previously parsed enums and consts)
        if let Some(&value) = self.global_values.get(name) {
            return self.try_i64_to_i32(value);
        }
        let (line, col) = self.current_position();
        Err(ParseError::UnresolvedEnumValue {
            name: name.to_string(),
            line,
            col,
        })
    }

    /// Extract the source text for a definition using the tracked start position.
    fn extract_definition_source(&self) -> String {
        let start_byte = self
            .tokens
            .get(self.def_start_pos)
            .map(|st| st.start)
            .unwrap_or(0);
        let end_byte = self.prev_end_byte();
        self.source_slice(start_byte, end_byte)
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
    #[error("{line}:{col}: default arms in unions are not supported")]
    UnsupportedDefaultArm { line: usize, col: usize },
    #[error("{line}:{col}: integer value {value} overflows target type")]
    IntegerOverflow { value: i64, line: usize, col: usize },
    #[error("{line}:{col}: unexpected #{directive} outside of #ifdef block")]
    UnexpectedDirective {
        directive: String,
        line: usize,
        col: usize,
    },
}

/// Result of `parse_type`: either a regular AST type or an anonymous union
/// that must be extracted into a named definition before entering the AST.
enum ParsedType {
    /// A regular type that can appear in the final AST.
    Type(Type),
    /// An inline `union switch (…) { … }` inside a struct. The parser
    /// converts this to a named `Union` definition via `extract_anonymous_union`
    /// and replaces it with a `Type::Ident` reference.
    AnonymousUnion {
        discriminant: UnionDiscriminant,
        arms: Vec<UnionArm>,
    },
}

/// State captured before parsing a type expression, used when extracting
/// anonymous unions into named definitions.
struct TypeParseContext {
    /// Byte offset where the type expression starts (for source extraction).
    start_byte: usize,
    /// Index into `extracted_definitions` before parsing (for parent fixup).
    extract_start_idx: usize,
}

/// Generate a nested type name from parent and field name.
fn generate_nested_type_name(parent: &str, field: &str) -> String {
    format!("{}{}", parent, field.to_upper_camel_case())
}
