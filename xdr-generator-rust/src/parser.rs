//! XDR parser using nom combinators.

use crate::ast::*;
use heck::ToUpperCamelCase;
use nom::{
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::char,
    combinator::{opt, recognize},
    multi::many0,
    sequence::{pair, preceded},
    IResult, Parser,
};
use std::collections::HashMap;
use thiserror::Error;

/// Parse XDR source text into an AST.
pub fn parse(source: &str) -> Result<XdrSpec, ParseError> {
    let mut ctx = ParseContext::new(source);
    ctx.parse()
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Nom(String),
}

/// Context for parsing, tracking extracted nested definitions and global values.
struct ParseContext<'a> {
    source: &'a str,
    extracted_definitions: Vec<Definition>,
    root_parent: Option<String>,
    global_values: HashMap<String, i64>,
}

impl<'a> ParseContext<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            extracted_definitions: Vec::new(),
            root_parent: None,
            global_values: HashMap::new(),
        }
    }

    fn parse(&mut self) -> Result<XdrSpec, ParseError> {
        let input = self.source;
        match self.parse_spec(input) {
            Ok((_, mut spec)) => {
                // Post-process to fix parent relationships for nested types.
                fix_parent_relationships(&mut spec.definitions);
                for ns in &mut spec.namespaces {
                    fix_parent_relationships(&mut ns.definitions);
                }
                Ok(spec)
            }
            Err(e) => Err(ParseError::Nom(format!("{:?}", e))),
        }
    }

    fn parse_spec<'b>(&mut self, input: &'b str) -> IResult<&'b str, XdrSpec> {
        let mut spec = XdrSpec::default();
        let mut remaining = input;

        // Skip leading whitespace
        let (rest, _) = ws(remaining)?;
        remaining = rest;

        while !remaining.is_empty() {
            // Skip extra semicolons at top level
            let (rest, _) = many0(preceded(ws, char(';'))).parse(remaining)?;
            remaining = rest;

            let (rest, _) = ws(remaining)?;
            remaining = rest;

            if remaining.is_empty() {
                break;
            }

            // Check for namespace
            if remaining.starts_with("namespace") {
                let (rest, ns) = self.parse_namespace(remaining)?;
                spec.namespaces.push(ns);
                remaining = rest;
            } else {
                // Track extracted definitions before parsing this definition
                let extract_start = self.extracted_definitions.len();

                let (rest, def) = self.parse_definition(remaining)?;
                remaining = rest;

                // Insert any newly extracted definitions before this definition
                for extracted in self.extracted_definitions.drain(extract_start..) {
                    spec.definitions.push(extracted);
                }

                spec.definitions.push(def);
            }
        }

        // Any remaining extracted definitions
        for extracted in self.extracted_definitions.drain(..) {
            spec.definitions.push(extracted);
        }

        Ok((remaining, spec))
    }

    fn parse_namespace<'b>(&mut self, input: &'b str) -> IResult<&'b str, Namespace> {
        let (rest, _) = tag("namespace")(input)?;
        let (rest, _) = ws(rest)?;
        let (rest, name) = ident(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char('{')(rest)?;

        let mut definitions = Vec::new();
        let mut remaining = rest;

        loop {
            let (rest, _) = ws(remaining)?;
            remaining = rest;

            // Skip extra semicolons
            let (rest, _) = many0(preceded(ws, char(';'))).parse(remaining)?;
            remaining = rest;

            let (rest, _) = ws(remaining)?;
            remaining = rest;

            if remaining.starts_with('}') {
                break;
            }

            // Track extracted definitions
            let extract_start = self.extracted_definitions.len();

            let (rest, def) = self.parse_definition(remaining)?;
            remaining = rest;

            // Insert extracted definitions before this definition
            for extracted in self.extracted_definitions.drain(extract_start..) {
                definitions.push(extracted);
            }

            definitions.push(def);
        }

        let (rest, _) = char('}')(remaining)?;

        Ok((
            rest,
            Namespace {
                name: name.to_string(),
                definitions,
            },
        ))
    }

    fn parse_definition<'b>(&mut self, input: &'b str) -> IResult<&'b str, Definition> {
        let (rest, _) = ws(input)?;
        let source_start = self.source.len() - rest.len();

        // Check which definition type
        if rest.starts_with("struct") {
            let (rest, def) = self.parse_struct(rest, source_start)?;
            Ok((rest, Definition::Struct(def)))
        } else if rest.starts_with("enum") {
            let (rest, def) = self.parse_enum(rest, source_start)?;
            Ok((rest, Definition::Enum(def)))
        } else if rest.starts_with("union") {
            let (rest, def) = self.parse_union(rest, source_start)?;
            Ok((rest, Definition::Union(def)))
        } else if rest.starts_with("typedef") {
            let (rest, def) = self.parse_typedef(rest, source_start)?;
            Ok((rest, Definition::Typedef(def)))
        } else if rest.starts_with("const") {
            let (rest, def) = self.parse_const(rest, source_start)?;
            Ok((rest, Definition::Const(def)))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                rest,
                nom::error::ErrorKind::Alt,
            )))
        }
    }

    fn parse_struct<'b>(
        &mut self,
        input: &'b str,
        source_start: usize,
    ) -> IResult<&'b str, Struct> {
        let (rest, _) = tag("struct")(input)?;
        let (rest, _) = ws(rest)?;
        let (rest, name) = ident(rest)?;
        let name = name.to_string();
        let (rest, _) = ws(rest)?;
        let (rest, _) = char('{')(rest)?;

        // Set root_parent for nested type name generation
        let prev_root = self.root_parent.take();
        self.root_parent = Some(name.clone());

        let mut members = Vec::new();
        let mut remaining = rest;

        loop {
            let (rest, _) = ws(remaining)?;
            remaining = rest;

            if remaining.starts_with('}') {
                break;
            }

            let (rest, member) = self.parse_member(remaining)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char(';')(rest)?;
            remaining = rest;
            members.push(member);
        }

        // Restore previous root_parent
        self.root_parent = prev_root;

        let (rest, _) = char('}')(remaining)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char(';')(rest)?;

        let source_end = self.source.len() - rest.len();
        let source = self.source[source_start..source_end].to_string();

        Ok((
            rest,
            Struct {
                name,
                members,
                source,
                is_nested: false,
                parent: None,
            },
        ))
    }

    fn parse_enum<'b>(&mut self, input: &'b str, source_start: usize) -> IResult<&'b str, Enum> {
        let (rest, _) = tag("enum")(input)?;
        let (rest, _) = ws(rest)?;
        let (rest, name) = ident(rest)?;
        let name = name.to_string();
        let (rest, _) = ws(rest)?;
        let (rest, _) = char('{')(rest)?;

        let mut members = Vec::new();
        let mut remaining = rest;

        loop {
            let (rest, _) = ws(remaining)?;
            remaining = rest;

            if remaining.starts_with('}') {
                break;
            }

            let (rest, member_name) = ident(remaining)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char('=')(rest)?;
            let (rest, _) = ws(rest)?;

            // Value can be integer or identifier reference
            let (rest, value) = if let Ok((rest, (val, _))) = int_literal(rest) {
                (rest, val as i32)
            } else {
                let (rest, ref_name) = ident(rest)?;
                let val = self.resolve_enum_value(ref_name, &members);
                (rest, val)
            };

            members.push(EnumMember {
                name: member_name.to_string(),
                value,
            });

            remaining = rest;

            // Check for comma
            let (rest, _) = ws(remaining)?;
            if rest.starts_with(',') {
                let (rest, _) = char(',')(rest)?;
                remaining = rest;
                // Check if next is closing brace (trailing comma)
                let (rest, _) = ws(remaining)?;
                if rest.starts_with('}') {
                    remaining = rest;
                    break;
                }
                remaining = rest;
            } else {
                remaining = rest;
                break;
            }
        }

        let (rest, _) = char('}')(remaining)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char(';')(rest)?;

        let source_end = self.source.len() - rest.len();
        let source = self.source[source_start..source_end].to_string();

        // Add all members to global_values for cross-enum resolution
        for m in &members {
            self.global_values.insert(m.name.clone(), m.value as i64);
        }

        Ok((
            rest,
            Enum {
                name,
                members,
                source,
            },
        ))
    }

    fn parse_union<'b>(&mut self, input: &'b str, source_start: usize) -> IResult<&'b str, Union> {
        let (rest, _) = tag("union")(input)?;
        let (rest, _) = ws(rest)?;
        let (rest, name) = ident(rest)?;
        let name = name.to_string();
        let (rest, _) = ws(rest)?;
        let (rest, _) = tag("switch")(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char('(')(rest)?;
        let (rest, _) = ws(rest)?;

        // Parse discriminant type and name
        let (rest, disc_type) = self.parse_type(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, disc_name) = ident(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char(')')(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char('{')(rest)?;

        // Set root_parent for inline struct extraction
        let prev_root = self.root_parent.take();
        self.root_parent = Some(name.clone());

        let mut arms = Vec::new();
        let mut default_arm = None;
        let mut remaining = rest;

        loop {
            let (rest, _) = ws(remaining)?;
            remaining = rest;

            if remaining.starts_with('}') {
                break;
            }

            let (rest, arm) = self.parse_union_arm(remaining)?;
            remaining = rest;

            if arm.cases.is_empty() {
                default_arm = Some(Box::new(arm));
            } else {
                arms.push(arm);
            }
        }

        // Restore previous root_parent
        self.root_parent = prev_root;

        let (rest, _) = char('}')(remaining)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char(';')(rest)?;

        let source_end = self.source.len() - rest.len();
        let source = self.source[source_start..source_end].to_string();

        Ok((
            rest,
            Union {
                name,
                discriminant: Discriminant {
                    name: disc_name.to_string(),
                    type_: disc_type,
                },
                arms,
                default_arm,
                source,
                is_nested: false,
                parent: None,
            },
        ))
    }

    fn parse_typedef<'b>(
        &mut self,
        input: &'b str,
        source_start: usize,
    ) -> IResult<&'b str, Typedef> {
        let (rest, _) = tag("typedef")(input)?;
        let (rest, _) = ws(rest)?;
        let (rest, type_) = self.parse_type(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, name) = ident(rest)?;
        let name = name.to_string();

        // Handle array suffix on typedef name
        let (rest, type_) = self.parse_type_suffix(rest, type_)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char(';')(rest)?;

        let source_end = self.source.len() - rest.len();
        let source = self.source[source_start..source_end].to_string();

        Ok((
            rest,
            Typedef {
                name,
                type_,
                source,
            },
        ))
    }

    fn parse_const<'b>(&mut self, input: &'b str, source_start: usize) -> IResult<&'b str, Const> {
        let (rest, _) = tag("const")(input)?;
        let (rest, _) = ws(rest)?;
        let (rest, name) = ident(rest)?;
        let name = name.to_string();
        let (rest, _) = ws(rest)?;
        let (rest, _) = char('=')(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, (value, base)) = int_literal(rest)?;
        let (rest, _) = ws(rest)?;
        let (rest, _) = char(';')(rest)?;

        let source_end = self.source.len() - rest.len();
        let source = self.source[source_start..source_end].to_string();

        // Add const value to global_values for enum reference resolution
        self.global_values.insert(name.clone(), value);

        Ok((
            rest,
            Const {
                name,
                value,
                base,
                source,
            },
        ))
    }

    fn parse_member<'b>(&mut self, input: &'b str) -> IResult<&'b str, Member> {
        let source_start = self.source.len() - input.len();
        let extract_start_idx = self.extracted_definitions.len();

        let (rest, type_) = self.parse_type(input)?;
        let type_end = self.source.len() - rest.len();
        let (rest, _) = ws(rest)?;
        let (rest, name) = ident(rest)?;
        let name = name.to_string();

        // Handle array suffix
        let (rest, type_) = self.parse_type_suffix(rest, type_)?;

        // If this is an anonymous union, extract it as a separate definition
        let type_ = match type_ {
            Type::AnonymousUnion {
                discriminant,
                arms,
                default_arm,
            } => {
                let union_name = if let Some(ref parent) = self.root_parent {
                    generate_nested_type_name(parent, &name)
                } else {
                    generate_nested_type_name(&name, "Union")
                };

                let source = self.source[source_start..type_end].to_string();

                let union_def = Union {
                    name: union_name.clone(),
                    discriminant: *discriminant,
                    arms,
                    default_arm,
                    source,
                    is_nested: true,
                    parent: self.root_parent.clone(),
                };

                // Fix up parent relationships for definitions extracted during union parsing
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

                self.extracted_definitions
                    .push(Definition::Union(union_def));
                Type::Ident(union_name)
            }
            other => other,
        };

        Ok((rest, Member { name, type_ }))
    }

    fn parse_union_arm<'b>(&mut self, input: &'b str) -> IResult<&'b str, UnionArm> {
        let mut cases = Vec::new();
        let mut remaining = input;

        // Parse case(s) - multiple cases can share the same arm
        loop {
            let (rest, _) = ws(remaining)?;
            remaining = rest;

            if remaining.starts_with("case") {
                let (rest, _) = tag("case")(remaining)?;
                let (rest, _) = ws(rest)?;

                let (rest, value) = if let Ok((rest, (val, _))) = int_literal(rest) {
                    (rest, CaseValue::Literal(val as i32))
                } else {
                    let (rest, name) = ident(rest)?;
                    (rest, CaseValue::Ident(name.to_string()))
                };

                let (rest, _) = ws(rest)?;
                let (rest, _) = char(':')(rest)?;
                remaining = rest;
                cases.push(UnionCase { value });
            } else if remaining.starts_with("default") {
                let (rest, _) = tag("default")(remaining)?;
                let (rest, _) = ws(rest)?;
                let (rest, _) = char(':')(rest)?;
                remaining = rest;
                break;
            } else {
                break;
            }
        }

        // Parse the arm type
        let (rest, _) = ws(remaining)?;
        remaining = rest;

        let (rest, type_) = if remaining.starts_with("void") {
            let (rest, _) = tag("void")(remaining)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char(';')(rest)?;
            (rest, None)
        } else if remaining.starts_with("struct") {
            // Inline struct definition - extract as a separate type
            let source_start = self.source.len() - remaining.len();
            let (rest, _) = tag("struct")(remaining)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char('{')(rest)?;

            // Find the field name by skipping the struct body first
            let start_rest = rest;
            let mut brace_depth = 1;
            let mut scan_rest = rest;
            while brace_depth > 0 {
                if scan_rest.starts_with('{') {
                    brace_depth += 1;
                    scan_rest = &scan_rest[1..];
                } else if scan_rest.starts_with('}') {
                    brace_depth -= 1;
                    scan_rest = &scan_rest[1..];
                } else if scan_rest.is_empty() {
                    break;
                } else {
                    scan_rest = &scan_rest[1..];
                }
            }

            let source_end = self.source.len() - scan_rest.len();
            let (rest2, _) = ws(scan_rest)?;
            let (rest2, field_name) = ident(rest2)?;
            let field_name = field_name.to_string();
            let (rest2, _) = ws(rest2)?;
            let (rest2, _) = char(';')(rest2)?;

            // Generate the struct name
            let struct_name = if let Some(ref parent) = self.root_parent {
                generate_nested_type_name(parent, &field_name)
            } else {
                field_name.to_upper_camel_case()
            };

            // Set root_parent to the struct name for any nested types
            let prev_root = self.root_parent.take();
            self.root_parent = Some(struct_name.clone());

            // Now parse the members
            let mut members = Vec::new();
            let mut remaining = start_rest;

            loop {
                let (rest, _) = ws(remaining)?;
                remaining = rest;

                if remaining.starts_with('}') {
                    break;
                }

                let (rest, member) = self.parse_member(remaining)?;
                let (rest, _) = ws(rest)?;
                let (rest, _) = char(';')(rest)?;
                remaining = rest;
                members.push(member);
            }

            // Skip the closing brace
            let (_, _) = char('}')(remaining)?;

            // Restore root_parent
            self.root_parent = prev_root;

            let source = self.source[source_start..source_end].to_string();

            let struct_def = Struct {
                name: struct_name.clone(),
                members,
                source,
                is_nested: true,
                parent: self.root_parent.clone(),
            };

            self.extracted_definitions
                .push(Definition::Struct(struct_def));
            (rest2, Some(Type::Ident(struct_name)))
        } else {
            let source_start = self.source.len() - remaining.len();
            let extract_start_idx = self.extracted_definitions.len();

            let (rest, type_) = self.parse_type(remaining)?;
            let type_end = self.source.len() - rest.len();
            let (rest, _) = ws(rest)?;
            let (rest, field_name) = ident(rest)?;
            let field_name = field_name.to_string();
            let (rest, type_) = self.parse_type_suffix(rest, type_)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char(';')(rest)?;

            // If this is an anonymous union, extract it
            let type_ = match type_ {
                Type::AnonymousUnion {
                    discriminant,
                    arms,
                    default_arm,
                } => {
                    let union_name = if let Some(ref parent) = self.root_parent {
                        generate_nested_type_name(parent, &field_name)
                    } else {
                        field_name.to_upper_camel_case()
                    };

                    let source = self.source[source_start..type_end].to_string();

                    let union_def = Union {
                        name: union_name.clone(),
                        discriminant: *discriminant,
                        arms,
                        default_arm,
                        source,
                        is_nested: true,
                        parent: self.root_parent.clone(),
                    };

                    for def in &mut self.extracted_definitions[extract_start_idx..] {
                        match def {
                            Definition::Struct(s)
                                if s.is_nested && s.parent == self.root_parent =>
                            {
                                s.parent = Some(union_name.clone());
                            }
                            Definition::Union(u) if u.is_nested && u.parent == self.root_parent => {
                                u.parent = Some(union_name.clone());
                            }
                            _ => {}
                        }
                    }

                    self.extracted_definitions
                        .push(Definition::Union(union_def));
                    Type::Ident(union_name)
                }
                other => other,
            };

            (rest, Some(type_))
        };

        Ok((rest, UnionArm { cases, type_ }))
    }

    fn parse_type<'b>(&mut self, input: &'b str) -> IResult<&'b str, Type> {
        let (rest, _) = ws(input)?;

        if rest.starts_with("int")
            && !rest[3..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let (rest, _) = tag("int")(rest)?;
            Ok((rest, Type::Int))
        } else if rest.starts_with("unsigned") {
            let (rest, _) = tag("unsigned")(rest)?;
            let (rest, _) = ws(rest)?;
            if rest.starts_with("int")
                && !rest[3..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
            {
                let (rest, _) = tag("int")(rest)?;
                Ok((rest, Type::UnsignedInt))
            } else if rest.starts_with("hyper")
                && !rest[5..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
            {
                let (rest, _) = tag("hyper")(rest)?;
                Ok((rest, Type::UnsignedHyper))
            } else {
                Err(nom::Err::Error(nom::error::Error::new(
                    rest,
                    nom::error::ErrorKind::Alt,
                )))
            }
        } else if rest.starts_with("hyper")
            && !rest[5..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let (rest, _) = tag("hyper")(rest)?;
            Ok((rest, Type::Hyper))
        } else if rest.starts_with("float")
            && !rest[5..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let (rest, _) = tag("float")(rest)?;
            Ok((rest, Type::Float))
        } else if rest.starts_with("double")
            && !rest[6..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let (rest, _) = tag("double")(rest)?;
            Ok((rest, Type::Double))
        } else if rest.starts_with("bool")
            && !rest[4..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let (rest, _) = tag("bool")(rest)?;
            Ok((rest, Type::Bool))
        } else if rest.starts_with("opaque")
            && !rest[6..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let (rest, _) = tag("opaque")(rest)?;
            self.parse_opaque_suffix(rest)
        } else if rest.starts_with("string")
            && !rest[6..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            let (rest, _) = tag("string")(rest)?;
            self.parse_string_suffix(rest)
        } else if rest.starts_with("union")
            && !rest[5..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            // Anonymous union inside struct
            let (rest, _) = tag("union")(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = tag("switch")(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char('(')(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, disc_type) = self.parse_type(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, disc_name) = ident(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char(')')(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char('{')(rest)?;

            let mut arms = Vec::new();
            let mut default_arm = None;
            let mut remaining = rest;

            loop {
                let (rest, _) = ws(remaining)?;
                remaining = rest;

                if remaining.starts_with('}') {
                    break;
                }

                let (rest, arm) = self.parse_union_arm(remaining)?;
                remaining = rest;

                if arm.cases.is_empty() {
                    default_arm = Some(Box::new(arm));
                } else {
                    arms.push(arm);
                }
            }

            let (rest, _) = char('}')(remaining)?;

            Ok((
                rest,
                Type::AnonymousUnion {
                    discriminant: Box::new(Discriminant {
                        name: disc_name.to_string(),
                        type_: disc_type,
                    }),
                    arms,
                    default_arm,
                },
            ))
        } else {
            // Try identifier
            let (rest, name) = ident(rest)?;
            // Handle built-in type aliases
            let base_type = match name {
                "uint64" => Type::UnsignedHyper,
                "int64" => Type::Hyper,
                "uint32" => Type::UnsignedInt,
                "int32" => Type::Int,
                "TRUE" | "FALSE" => Type::Bool,
                _ => Type::Ident(name.to_string()),
            };

            // Check for optional type suffix (Type* field)
            let (rest, _) = ws(rest)?;
            if rest.starts_with('*') {
                let (rest, _) = char('*')(rest)?;
                Ok((rest, Type::Optional(Box::new(base_type))))
            } else {
                Ok((rest, base_type))
            }
        }
    }

    fn parse_type_suffix<'b>(&mut self, input: &'b str, base: Type) -> IResult<&'b str, Type> {
        let (rest, _) = ws(input)?;

        if rest.starts_with('[') {
            let (rest, _) = char('[')(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, size) = self.parse_size(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char(']')(rest)?;

            // Special case: opaque name[size] or string name[size]
            match base {
                Type::OpaqueVar(None) => Ok((rest, Type::OpaqueFixed(size))),
                Type::String(None) => Ok((rest, Type::OpaqueFixed(size))),
                _ => Ok((
                    rest,
                    Type::Array {
                        element_type: Box::new(base),
                        size,
                    },
                )),
            }
        } else if rest.starts_with('<') {
            let (rest, _) = char('<')(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, max) = if rest.starts_with('>') {
                (rest, None)
            } else {
                let (rest, size) = self.parse_size(rest)?;
                (rest, Some(size))
            };
            let (rest, _) = ws(rest)?;
            let (rest, _) = char('>')(rest)?;

            // Special case: opaque name<max> or string name<max>
            match base {
                Type::OpaqueVar(None) => Ok((rest, Type::OpaqueVar(max))),
                Type::String(None) => Ok((rest, Type::String(max))),
                _ => Ok((
                    rest,
                    Type::VarArray {
                        element_type: Box::new(base),
                        max_size: max,
                    },
                )),
            }
        } else if rest.starts_with('*') {
            let (rest, _) = char('*')(rest)?;
            Ok((rest, Type::Optional(Box::new(base))))
        } else {
            Ok((rest, base))
        }
    }

    fn parse_opaque_suffix<'b>(&mut self, input: &'b str) -> IResult<&'b str, Type> {
        let (rest, _) = ws(input)?;

        if rest.starts_with('[') {
            let (rest, _) = char('[')(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, size) = self.parse_size(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, _) = char(']')(rest)?;
            Ok((rest, Type::OpaqueFixed(size)))
        } else if rest.starts_with('<') {
            let (rest, _) = char('<')(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, max) = if rest.starts_with('>') {
                (rest, None)
            } else {
                let (rest, size) = self.parse_size(rest)?;
                (rest, Some(size))
            };
            let (rest, _) = ws(rest)?;
            let (rest, _) = char('>')(rest)?;
            Ok((rest, Type::OpaqueVar(max)))
        } else {
            // Bare opaque - variable with no max
            Ok((rest, Type::OpaqueVar(None)))
        }
    }

    fn parse_string_suffix<'b>(&mut self, input: &'b str) -> IResult<&'b str, Type> {
        let (rest, _) = ws(input)?;

        if rest.starts_with('<') {
            let (rest, _) = char('<')(rest)?;
            let (rest, _) = ws(rest)?;
            let (rest, max) = if rest.starts_with('>') {
                (rest, None)
            } else {
                let (rest, size) = self.parse_size(rest)?;
                (rest, Some(size))
            };
            let (rest, _) = ws(rest)?;
            let (rest, _) = char('>')(rest)?;
            Ok((rest, Type::String(max)))
        } else {
            Ok((rest, Type::String(None)))
        }
    }

    fn parse_size<'b>(&mut self, input: &'b str) -> IResult<&'b str, Size> {
        if let Ok((rest, (value, _))) = int_literal(input) {
            Ok((rest, Size::Literal(value as u32)))
        } else {
            let (rest, name) = ident(input)?;
            Ok((rest, Size::Named(name.to_string())))
        }
    }

    fn resolve_enum_value(&self, name: &str, members: &[EnumMember]) -> i32 {
        // First check if it's in the current enum being parsed
        for m in members {
            if m.name == name {
                return m.value;
            }
        }
        // Check global values (previously parsed enums and consts)
        if let Some(&value) = self.global_values.get(name) {
            return value as i32;
        }
        // Return 0 as fallback
        0
    }
}

// ============================================================================
// Basic parsers
// ============================================================================

/// Skip whitespace and comments (line, block, and preprocessor directives).
fn ws(input: &str) -> IResult<&str, ()> {
    let mut rest = input;

    loop {
        // Skip whitespace
        let (remaining, _) = take_while(|c: char| c.is_whitespace())(rest)?;
        rest = remaining;

        if rest.is_empty() {
            break;
        }

        // Line comment
        if rest.starts_with("//") {
            let (remaining, _) = take_while(|c| c != '\n')(rest)?;
            rest = remaining;
            continue;
        }

        // Block comment
        if rest.starts_with("/*") {
            let (remaining, _) = tag("/*")(rest)?;
            let (remaining, _) = take_until("*/")(remaining)?;
            let (remaining, _) = tag("*/")(remaining)?;
            rest = remaining;
            continue;
        }

        // Preprocessor directive
        if rest.starts_with('%') {
            let (remaining, _) = take_while(|c| c != '\n')(rest)?;
            rest = remaining;
            continue;
        }

        break;
    }

    Ok((rest, ()))
}

/// Parse an identifier.
fn ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(|c: char| c.is_alphabetic() || c == '_'),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))
    .parse(input)
}

/// Parse an integer literal (decimal or hex), returning value and base.
fn int_literal(input: &str) -> IResult<&str, (i64, IntBase)> {
    if input.starts_with("0x") || input.starts_with("0X") {
        // Hex
        let (rest, _) =
            tag("0x")(input).or_else(|_: nom::Err<nom::error::Error<&str>>| tag("0X")(input))?;
        let (rest, digits) = take_while1(|c: char| c.is_ascii_hexdigit())(rest)?;
        let value = i64::from_str_radix(digits, 16).map_err(|_| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
        })?;
        Ok((rest, (value, IntBase::Hexadecimal)))
    } else {
        // Decimal (possibly negative)
        let (rest, neg) = opt(char('-')).parse(input)?;
        let (rest, digits) = take_while1(|c: char| c.is_ascii_digit())(rest)?;
        let value: i64 = digits.parse().map_err(|_| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
        })?;
        let value = if neg.is_some() { -value } else { value };
        Ok((rest, (value, IntBase::Decimal)))
    }
}

// ============================================================================
// Helper functions
// ============================================================================

/// Fix parent relationships for nested types based on naming patterns.
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_struct() {
        let input = "struct Foo { int x; unsigned hyper y; };";
        let spec = parse(input).unwrap();

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
        let spec = parse(input).unwrap();

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
        let spec = parse(input).unwrap();

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
        let spec = parse(input).unwrap();

        assert_eq!(spec.namespaces.len(), 1);
        assert_eq!(spec.namespaces[0].name, "stellar");
        assert_eq!(spec.namespaces[0].definitions.len(), 1);
    }

    #[test]
    fn test_parse_hex() {
        let input = "const KEY_TYPE_MUXED = 0x100;";
        let spec = parse(input).unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Const(c) = &spec.definitions[0] {
            assert_eq!(c.name, "KEY_TYPE_MUXED");
            assert_eq!(c.value, 256);
            assert_eq!(c.base, IntBase::Hexadecimal);
        } else {
            panic!("Expected const");
        }
    }

    #[test]
    fn test_parse_negative_number() {
        let input = "const FOO = -1;";
        let spec = parse(input).unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Const(c) = &spec.definitions[0] {
            assert_eq!(c.name, "FOO");
            assert_eq!(c.value, -1);
            assert_eq!(c.base, IntBase::Decimal);
        } else {
            panic!("Expected const");
        }
    }

    #[test]
    fn test_comments() {
        let input = r#"
        // line comment
        struct /* block comment */ Foo { };
        "#;
        let spec = parse(input).unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Struct(s) = &spec.definitions[0] {
            assert_eq!(s.name, "Foo");
        } else {
            panic!("Expected struct");
        }
    }

    #[test]
    fn test_preprocessor() {
        let input = "%#include \"xdr.h\"\nstruct Foo {};";
        let spec = parse(input).unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Struct(s) = &spec.definitions[0] {
            assert_eq!(s.name, "Foo");
        } else {
            panic!("Expected struct");
        }
    }
}
