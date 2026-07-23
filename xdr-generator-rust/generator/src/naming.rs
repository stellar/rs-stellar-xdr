use heck::{ToSnakeCase, ToUpperCamelCase};
use xdr_parser::ast::UnionCaseValue;

/// Convert an XDR name to a Rust type name (UpperCamelCase).
pub(crate) fn type_name(name: &str) -> String {
    escape_type_name(name).to_upper_camel_case()
}

/// Convert an XDR name to a Rust module name (snake_case).
pub(crate) fn mod_name(name: &str) -> String {
    escape_type_name(name).to_snake_case()
}

/// Convert an XDR name to a Rust field name (snake_case).
pub(crate) fn field_name(name: &str) -> String {
    let snake = name.to_snake_case();
    escape_field_name(&snake)
}

/// When a field name is keyword-escaped to a raw identifier (e.g. `type` ->
/// `r#type`), serde derives the JSON key from the bare keyword (`type`). Prior
/// versions escaped the field with a trailing underscore (`type_`) and
/// serialized it as such, so return that legacy key to keep accepting it on
/// deserialization via `#[serde(alias = ...)]`. Returns `None` when no escaping
/// occurred.
pub(crate) fn field_serde_alias(name: &str) -> Option<String> {
    let snake = name.to_snake_case();
    if escape_field_name(&snake).starts_with("r#") {
        Some(format!("{snake}_"))
    } else {
        None
    }
}

fn escape_type_name(name: &str) -> String {
    match name {
        "type" => "type_".to_string(),
        "Error" => "SError".to_string(),
        _ => name.to_string(),
    }
}

fn escape_field_name(name: &str) -> String {
    match name {
        // Emit the raw identifier so the Rust field is literally named `type`
        // and serde/schemars derive the JSON key `type` from it directly.
        "type" => "r#type".to_string(),
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
    } else if let UnionCaseValue::Literal(n) = value {
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
