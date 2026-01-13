//! XDR parser using pest.

use crate::ast::*;
use heck::ToUpperCamelCase;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "xdr.pest"]
struct XdrParser;

/// The base (radix) of an integer literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntBase {
    Decimal,
    Hexadecimal,
}

/// Parse XDR source text into an AST.
pub fn parse(source: &str) -> Result<XdrSpec, ParseError> {
    let mut pairs = XdrParser::parse(Rule::spec, source)?;
    let spec_pair = pairs.next().ok_or(ParseError::UnexpectedEof)?;

    let mut builder = AstBuilder::new(source);
    builder.build_spec(spec_pair)
}

struct AstBuilder<'a> {
    source: &'a str,
    /// Extracted nested type definitions (anonymous unions, inline structs)
    extracted_definitions: Vec<Definition>,
    /// Root parent type name for generating nested type names
    root_parent: Option<String>,
    /// Global map of enum member names and const names to their values
    global_values: HashMap<String, i64>,
}

impl<'a> AstBuilder<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            extracted_definitions: Vec::new(),
            root_parent: None,
            global_values: HashMap::new(),
        }
    }

    fn build_spec(&mut self, pair: pest::iterators::Pair<'a, Rule>) -> Result<XdrSpec, ParseError> {
        let mut spec = XdrSpec::default();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::namespace => {
                    let ns = self.build_namespace(inner)?;
                    spec.namespaces.push(ns);
                }
                Rule::definition => {
                    // Track extracted definitions before parsing this definition
                    let extract_start = self.extracted_definitions.len();

                    if let Some(def) = self.build_definition(inner)? {
                        // Insert any newly extracted definitions before this definition
                        for extracted in self.extracted_definitions.drain(extract_start..) {
                            spec.definitions.push(extracted);
                        }
                        spec.definitions.push(def);
                    }
                }
                Rule::EOI => {}
                _ => {}
            }
        }

        // Any remaining extracted definitions
        for extracted in self.extracted_definitions.drain(..) {
            spec.definitions.push(extracted);
        }

        // Post-process to fix parent relationships for nested types
        fix_parent_relationships(&mut spec.definitions);
        for ns in &mut spec.namespaces {
            fix_parent_relationships(&mut ns.definitions);
        }

        Ok(spec)
    }

    fn build_namespace(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Namespace, ParseError> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        let mut definitions = Vec::new();
        for def_pair in inner {
            if def_pair.as_rule() == Rule::definition {
                let extract_start = self.extracted_definitions.len();
                if let Some(def) = self.build_definition(def_pair)? {
                    for extracted in self.extracted_definitions.drain(extract_start..) {
                        definitions.push(extracted);
                    }
                    definitions.push(def);
                }
            }
        }

        Ok(Namespace { name, definitions })
    }

    fn build_definition(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Option<Definition>, ParseError> {
        let inner = pair.into_inner().next();
        let inner = match inner {
            Some(p) => p,
            None => return Ok(None), // Empty definition (just semicolon)
        };

        let def = match inner.as_rule() {
            Rule::struct_def => Definition::Struct(self.build_struct(inner)?),
            Rule::enum_def => Definition::Enum(self.build_enum(inner)?),
            Rule::union_def => Definition::Union(self.build_union(inner)?),
            Rule::typedef_def => Definition::Typedef(self.build_typedef(inner)?),
            Rule::const_def => Definition::Const(self.build_const(inner)?),
            _ => return Ok(None),
        };

        Ok(Some(def))
    }

    fn build_struct(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Struct, ParseError> {
        let source = pair.as_str().to_string();
        let mut inner = pair.into_inner();

        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        // Set root_parent for nested type name generation
        let prev_root = self.root_parent.take();
        self.root_parent = Some(name.clone());

        let mut members = Vec::new();
        for member_pair in inner {
            if member_pair.as_rule() == Rule::struct_member {
                members.push(self.build_struct_member(member_pair)?);
            }
        }

        // Restore previous root_parent
        self.root_parent = prev_root;

        Ok(Struct {
            name,
            members,
            source,
            is_nested: false,
            parent: None,
        })
    }

    fn build_struct_member(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Member, ParseError> {
        let source_start = pair.as_span().start();
        let mut inner = pair.into_inner();

        // Track extracted definitions before parsing type
        let extract_start_idx = self.extracted_definitions.len();

        let type_pair = inner.next().ok_or(ParseError::UnexpectedEof)?;
        let type_source_end = type_pair.as_span().end();
        let mut type_ = self.build_type(type_pair)?;

        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        // Handle type suffix if present
        if let Some(suffix_pair) = inner.next() {
            type_ = self.apply_type_suffix(type_, suffix_pair)?;
        }

        // If this is an anonymous union, extract it as a separate definition
        let type_ = self.extract_anonymous_union_if_needed(
            type_,
            &name,
            source_start,
            type_source_end,
            extract_start_idx,
        );

        Ok(Member { name, type_ })
    }

    fn build_enum(&mut self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Enum, ParseError> {
        let source = pair.as_str().to_string();
        let mut inner = pair.into_inner();

        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        let mut members = Vec::new();
        for members_pair in inner {
            if members_pair.as_rule() == Rule::enum_members {
                for member_pair in members_pair.into_inner() {
                    if member_pair.as_rule() == Rule::enum_member {
                        members.push(self.build_enum_member(member_pair, &members)?);
                    }
                }
            }
        }

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

    fn build_enum_member(
        &self,
        pair: pest::iterators::Pair<'a, Rule>,
        existing_members: &[EnumMember],
    ) -> Result<EnumMember, ParseError> {
        let mut inner = pair.into_inner();

        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();
        let value_pair = inner.next().ok_or(ParseError::UnexpectedEof)?;

        let value = self.parse_enum_value(value_pair, existing_members)?;

        Ok(EnumMember { name, value })
    }

    fn parse_enum_value(
        &self,
        pair: pest::iterators::Pair<'a, Rule>,
        existing_members: &[EnumMember],
    ) -> Result<i32, ParseError> {
        let inner = pair.into_inner().next().ok_or(ParseError::UnexpectedEof)?;

        match inner.as_rule() {
            Rule::int_literal => {
                let (value, _) = parse_int_literal(inner.as_str());
                Ok(value as i32)
            }
            Rule::ident => {
                let ref_name = inner.as_str();
                Ok(self.resolve_enum_value(ref_name, existing_members))
            }
            _ => Err(ParseError::UnexpectedRule(format!("{:?}", inner.as_rule()))),
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

    fn build_union(&mut self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Union, ParseError> {
        let source = pair.as_str().to_string();
        let mut inner = pair.into_inner();

        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        // Parse discriminant type
        let disc_type_pair = inner.next().ok_or(ParseError::UnexpectedEof)?;
        let disc_type = self.build_type(disc_type_pair)?;

        // Parse discriminant name
        let disc_name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        // Set root_parent for inline struct extraction
        let prev_root = self.root_parent.take();
        self.root_parent = Some(name.clone());

        let mut arms = Vec::new();
        let mut default_arm = None;

        for arm_pair in inner {
            if arm_pair.as_rule() == Rule::union_arm {
                let arm = self.build_union_arm(arm_pair)?;
                if arm.cases.is_empty() {
                    default_arm = Some(Box::new(arm));
                } else {
                    arms.push(arm);
                }
            }
        }

        // Restore previous root_parent
        self.root_parent = prev_root;

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

    fn build_union_arm(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<UnionArm, ParseError> {
        let mut cases = Vec::new();
        let mut arm_type = None;

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::union_cases => {
                    for case_pair in inner.into_inner() {
                        match case_pair.as_rule() {
                            Rule::case_clause => {
                                let value_pair = case_pair
                                    .into_inner()
                                    .next()
                                    .ok_or(ParseError::UnexpectedEof)?;
                                let value = self.build_case_value(value_pair)?;
                                cases.push(UnionCase { value });
                            }
                            Rule::default_clause => {
                                // Default has no cases - cases stays empty
                            }
                            _ => {}
                        }
                    }
                }
                Rule::union_arm_type => {
                    arm_type = Some(self.build_union_arm_type(inner)?);
                }
                _ => {}
            }
        }

        Ok(UnionArm {
            cases,
            type_: arm_type.flatten(),
        })
    }

    fn build_case_value(
        &self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<CaseValue, ParseError> {
        let inner = pair.into_inner().next().ok_or(ParseError::UnexpectedEof)?;

        match inner.as_rule() {
            Rule::int_literal => {
                let (value, _) = parse_int_literal(inner.as_str());
                Ok(CaseValue::Literal(value as i32))
            }
            Rule::ident => Ok(CaseValue::Ident(inner.as_str().to_string())),
            _ => Err(ParseError::UnexpectedRule(format!("{:?}", inner.as_rule()))),
        }
    }

    fn build_union_arm_type(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Option<Type>, ParseError> {
        let inner = pair.into_inner().next().ok_or(ParseError::UnexpectedEof)?;

        match inner.as_rule() {
            Rule::void_arm => Ok(None),
            Rule::inline_struct_arm => {
                let source = inner.as_str();
                let arm_inner = inner.into_inner();

                // Collect members first
                let mut member_pairs: Vec<pest::iterators::Pair<'a, Rule>> = Vec::new();
                let mut field_name = String::new();

                for p in arm_inner {
                    match p.as_rule() {
                        Rule::struct_member => member_pairs.push(p),
                        Rule::ident => field_name = p.as_str().to_string(),
                        _ => {}
                    }
                }

                // Generate the struct name
                let struct_name = if let Some(ref parent) = self.root_parent {
                    generate_nested_type_name(parent, &field_name)
                } else {
                    field_name.to_upper_camel_case()
                };

                // Set root_parent to the struct name for any nested types
                let prev_root = self.root_parent.take();
                self.root_parent = Some(struct_name.clone());

                // Parse members
                let mut members = Vec::new();
                for member_pair in member_pairs {
                    members.push(self.build_struct_member(member_pair)?);
                }

                // Restore root_parent
                self.root_parent = prev_root;

                // Extract source text (approximate - from struct keyword to closing brace)
                let struct_source = extract_inline_struct_source(source);

                // Create the struct definition
                let struct_def = Struct {
                    name: struct_name.clone(),
                    members,
                    source: struct_source,
                    is_nested: true,
                    parent: self.root_parent.clone(),
                };

                self.extracted_definitions
                    .push(Definition::Struct(struct_def));

                Ok(Some(Type::Ident(struct_name)))
            }
            Rule::type_arm => {
                let source_start = inner.as_span().start();
                let mut arm_inner = inner.into_inner();

                // Track extracted definitions before parsing type
                let extract_start_idx = self.extracted_definitions.len();

                let type_pair = arm_inner.next().ok_or(ParseError::UnexpectedEof)?;
                let type_source_end = type_pair.as_span().end();
                let mut type_ = self.build_type(type_pair)?;

                let field_name = arm_inner
                    .next()
                    .ok_or(ParseError::UnexpectedEof)?
                    .as_str()
                    .to_string();

                // Handle type suffix if present
                if let Some(suffix_pair) = arm_inner.next() {
                    type_ = self.apply_type_suffix(type_, suffix_pair)?;
                }

                // Extract anonymous union if needed
                let type_ = self.extract_anonymous_union_if_needed(
                    type_,
                    &field_name,
                    source_start,
                    type_source_end,
                    extract_start_idx,
                );

                Ok(Some(type_))
            }
            _ => Err(ParseError::UnexpectedRule(format!("{:?}", inner.as_rule()))),
        }
    }

    fn build_typedef(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Typedef, ParseError> {
        let source = pair.as_str().to_string();
        let mut inner = pair.into_inner();

        let type_pair = inner.next().ok_or(ParseError::UnexpectedEof)?;
        let mut type_ = self.build_type(type_pair)?;

        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        // Handle type suffix if present
        if let Some(suffix_pair) = inner.next() {
            type_ = self.apply_type_suffix(type_, suffix_pair)?;
        }

        Ok(Typedef {
            name,
            type_,
            source,
        })
    }

    fn build_const(&mut self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Const, ParseError> {
        let source = pair.as_str().to_string();
        let mut inner = pair.into_inner();

        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        let value_pair = inner.next().ok_or(ParseError::UnexpectedEof)?;
        let (value, base) = parse_int_literal(value_pair.as_str());

        // Add const value to global_values for enum reference resolution
        self.global_values.insert(name.clone(), value);

        Ok(Const {
            name,
            value,
            base,
            source,
        })
    }

    fn build_type(&mut self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Type, ParseError> {
        let inner = pair.into_inner().next().ok_or(ParseError::UnexpectedEof)?;

        match inner.as_rule() {
            Rule::int_type => Ok(Type::Int),
            Rule::unsigned_type => {
                let type_keyword = inner.as_str();
                if type_keyword.contains("hyper") {
                    Ok(Type::UnsignedHyper)
                } else {
                    Ok(Type::UnsignedInt)
                }
            }
            Rule::hyper_type => Ok(Type::Hyper),
            Rule::float_type => Ok(Type::Float),
            Rule::double_type => Ok(Type::Double),
            Rule::bool_type => Ok(Type::Bool),
            Rule::opaque_type => self.build_opaque_type(inner),
            Rule::string_type => self.build_string_type(inner),
            Rule::anonymous_union => self.build_anonymous_union(inner),
            Rule::ident_type => self.build_ident_type(inner),
            _ => Err(ParseError::UnexpectedRule(format!("{:?}", inner.as_rule()))),
        }
    }

    fn build_opaque_type(&self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Type, ParseError> {
        let inner = pair.into_inner().next();

        match inner {
            Some(suffix) => match suffix.as_rule() {
                Rule::fixed_size => {
                    let size = self.build_size(
                        suffix
                            .into_inner()
                            .next()
                            .ok_or(ParseError::UnexpectedEof)?,
                    )?;
                    Ok(Type::OpaqueFixed(size))
                }
                Rule::var_size => {
                    let size = suffix
                        .into_inner()
                        .next()
                        .map(|p| self.build_size(p))
                        .transpose()?;
                    Ok(Type::OpaqueVar(size))
                }
                _ => Ok(Type::OpaqueVar(None)),
            },
            None => Ok(Type::OpaqueVar(None)),
        }
    }

    fn build_string_type(&self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Type, ParseError> {
        let inner = pair.into_inner().next();

        match inner {
            Some(suffix) if suffix.as_rule() == Rule::var_size => {
                let size = suffix
                    .into_inner()
                    .next()
                    .map(|p| self.build_size(p))
                    .transpose()?;
                Ok(Type::String(size))
            }
            _ => Ok(Type::String(None)),
        }
    }

    fn build_anonymous_union(
        &mut self,
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Type, ParseError> {
        let mut inner = pair.into_inner();

        // Parse discriminant type
        let disc_type_pair = inner.next().ok_or(ParseError::UnexpectedEof)?;
        let disc_type = self.build_type(disc_type_pair)?;

        // Parse discriminant name
        let disc_name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        let mut arms = Vec::new();
        let mut default_arm = None;

        for arm_pair in inner {
            if arm_pair.as_rule() == Rule::union_arm {
                let arm = self.build_union_arm(arm_pair)?;
                if arm.cases.is_empty() {
                    default_arm = Some(Box::new(arm));
                } else {
                    arms.push(arm);
                }
            }
        }

        Ok(Type::AnonymousUnion {
            discriminant: Box::new(Discriminant {
                name: disc_name,
                type_: disc_type,
            }),
            arms,
            default_arm,
        })
    }

    fn build_ident_type(&self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Type, ParseError> {
        let mut inner = pair.into_inner();
        let name = inner
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .as_str()
            .to_string();

        // Handle built-in type aliases
        let base_type = match name.as_str() {
            "uint64" => Type::UnsignedHyper,
            "int64" => Type::Hyper,
            "uint32" => Type::UnsignedInt,
            "int32" => Type::Int,
            "TRUE" | "FALSE" => Type::Bool,
            _ => Type::Ident(name),
        };

        // Check for optional marker (*)
        if inner.next().is_some() {
            Ok(Type::Optional(Box::new(base_type)))
        } else {
            Ok(base_type)
        }
    }

    fn apply_type_suffix(
        &self,
        base: Type,
        suffix_pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Type, ParseError> {
        let inner = suffix_pair
            .into_inner()
            .next()
            .ok_or(ParseError::UnexpectedEof)?;

        match inner.as_rule() {
            Rule::fixed_size => {
                let size =
                    self.build_size(inner.into_inner().next().ok_or(ParseError::UnexpectedEof)?)?;

                // Special case: opaque name[size] or string name[size]
                match base {
                    Type::OpaqueVar(None) => Ok(Type::OpaqueFixed(size)),
                    Type::String(None) => Ok(Type::OpaqueFixed(size)),
                    _ => Ok(Type::Array {
                        element_type: Box::new(base),
                        size,
                    }),
                }
            }
            Rule::var_size => {
                let max = inner
                    .into_inner()
                    .next()
                    .map(|p| self.build_size(p))
                    .transpose()?;

                // Special case: opaque name<max> or string name<max>
                match base {
                    Type::OpaqueVar(None) => Ok(Type::OpaqueVar(max)),
                    Type::String(None) => Ok(Type::String(max)),
                    _ => Ok(Type::VarArray {
                        element_type: Box::new(base),
                        max_size: max,
                    }),
                }
            }
            Rule::optional_marker => Ok(Type::Optional(Box::new(base))),
            _ => Ok(base),
        }
    }

    fn build_size(&self, pair: pest::iterators::Pair<'a, Rule>) -> Result<Size, ParseError> {
        let inner = pair.into_inner().next().ok_or(ParseError::UnexpectedEof)?;

        match inner.as_rule() {
            Rule::int_literal => {
                let (value, _) = parse_int_literal(inner.as_str());
                Ok(Size::Literal(value as u32))
            }
            Rule::ident => Ok(Size::Named(inner.as_str().to_string())),
            _ => Err(ParseError::UnexpectedRule(format!("{:?}", inner.as_rule()))),
        }
    }

    fn extract_anonymous_union_if_needed(
        &mut self,
        type_: Type,
        field_name: &str,
        source_start: usize,
        source_end: usize,
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
                let source = if source_start < source_end && source_end <= self.source.len() {
                    self.source[source_start..source_end].to_string()
                } else {
                    String::new()
                };

                // Create the Union definition
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
}

/// Parse an integer literal, returning both the value and whether it was in hex format.
fn parse_int_literal(s: &str) -> (i64, IntBase) {
    if s.starts_with("0x") || s.starts_with("0X") {
        let value = i64::from_str_radix(&s[2..], 16).unwrap_or(0);
        (value, IntBase::Hexadecimal)
    } else {
        let value = s.parse::<i64>().unwrap_or(0);
        (value, IntBase::Decimal)
    }
}

/// Extract source text for an inline struct (approximate).
fn extract_inline_struct_source(full_source: &str) -> String {
    // Find the "struct {" part and extract up to the closing "}"
    if let Some(start) = full_source.find("struct") {
        if let Some(end) = full_source.rfind('}') {
            return full_source[start..=end].to_string();
        }
    }
    String::new()
}

/// Generate a nested type name from parent and field name.
fn generate_nested_type_name(parent: &str, field: &str) -> String {
    format!("{}{}", parent, field.to_upper_camel_case())
}

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

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("pest parse error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("unexpected rule: {0}")]
    UnexpectedRule(String),
}

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
    fn test_parse_const_hex() {
        let input = "const FOO = 0x100;";
        let spec = parse(input).unwrap();

        assert_eq!(spec.definitions.len(), 1);
        if let Definition::Const(c) = &spec.definitions[0] {
            assert_eq!(c.name, "FOO");
            assert_eq!(c.value, 256);
            assert_eq!(c.base, IntBase::Hexadecimal);
        } else {
            panic!("Expected const");
        }
    }

    #[test]
    fn test_parse_const_negative() {
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
}
