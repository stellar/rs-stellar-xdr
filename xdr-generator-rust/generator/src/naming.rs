use heck::{ToSnakeCase, ToUpperCamelCase};
use xdr_parser::ast::UnionCaseValue;

/// Convert an XDR name to a Rust type name (UpperCamelCase).
pub(crate) fn type_name(name: &str) -> String {
    escape_type_name(name).to_upper_camel_case()
}

/// Convert an XDR name to a Rust field name (snake_case).
pub(crate) fn field_name(name: &str) -> String {
    let snake = name.to_snake_case();
    escape_field_name(&snake)
}

fn escape_type_name(name: &str) -> String {
    match name {
        "Error" => "SError".to_string(),
        _ => name.to_string(),
    }
}

fn escape_field_name(name: &str) -> String {
    match name {
        "type" => "type_".to_string(),
        _ => name.to_string(),
    }
}

/// Format a source comment for Rust `///` documentation.
pub(crate) fn source_comment(source: &str, kind: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let trimmed = source.trim();
    let lines: Vec<&str> = trimmed.lines().collect();
    let formatted: Vec<String> = lines.iter().map(|l| format!("/// {l}")).collect();
    format!(
        " is an XDR {kind} defined as:\n///\n/// ```text\n{}\n/// ```\n///",
        formatted.join("\n")
    )
}

/// Format a union case value: returns (case_name, case_value_expr).
pub(crate) fn case_value(
    discriminant_type: &str,
    is_builtin: bool,
    value: &UnionCaseValue,
    prefix: &str,
) -> (String, String) {
    if let Some(stripped) = value.stripped_ident(prefix) {
        let name = type_name(&stripped);
        let val = if is_builtin {
            name.clone()
        } else {
            format!("{discriminant_type}::{name}")
        };
        (name, val)
    } else if let UnionCaseValue::Literal { literal: n } = value {
        let case_name = format!("V{n}");
        let val = if is_builtin {
            n.to_string()
        } else {
            format!("{discriminant_type}({n})")
        };
        (case_name, val)
    } else {
        unreachable!()
    }
}
