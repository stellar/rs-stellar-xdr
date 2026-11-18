use heck::{ToSnakeCase, ToUpperCamelCase};
use xdr_parser::ast::UnionCaseValue;

/// Convert an XDR name to a Rust type name (`UpperCamelCase`).
pub(crate) fn type_name(name: &str) -> String {
    escape_type_name(&fix_acronyms(name)).to_upper_camel_case()
}

/// Convert an XDR name to a Rust module name (`snake_case`).
pub(crate) fn mod_name(name: &str) -> String {
    escape_type_name(&fix_acronyms(name)).to_snake_case()
}

/// Convert an XDR name to a Rust constant name (`SCREAMING_SNAKE_CASE`).
pub(crate) fn const_name(name: &str) -> String {
    field_name(name).to_uppercase()
}

/// Convert an XDR name to a Rust field name (`snake_case`).
pub(crate) fn field_name(name: &str) -> String {
    let snake = fix_acronyms(name).to_snake_case();
    escape_field_name(&snake)
}

/// If the Rust field name for an XDR field was keyword-escaped away from its
/// plain `snake_case` name (e.g. `type` -> `type_`), return the correct JSON key
/// (the unescaped `snake_case` name). Returns `None` when no escaping occurred.
pub(crate) fn field_json_rename(name: &str) -> Option<String> {
    let snake = fix_acronyms(name).to_snake_case();
    if escape_field_name(&snake) == snake {
        None
    } else {
        Some(snake)
    }
}

/// Acronyms that are written with a lowercase letter directly after them, and
/// the spelling to use instead. heck would otherwise treat the last capital as
/// the start of the next word, e.g. `IPv4` -> `i_pv4`.
const ACRONYM_SPELLINGS: &[(&str, &str)] = &[("IPv", "Ipv")];

/// Rewrite acronyms that heck splits in the wrong place, so that heck sees
/// them as a single word:
///
/// - Known spellings in [`ACRONYM_SPELLINGS`], e.g. `IPv4` -> `Ipv4`.
/// - Plural acronyms, e.g. `IDs` -> `Ids`, where heck would otherwise produce
///   `i_ds`.
///
/// A match only counts at the start of a word, i.e. not directly after another
/// capital letter.
fn fix_acronyms(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len());
    let mut i = 0;
    'outer: while i < chars.len() {
        let at_word_start = i == 0 || !chars[i - 1].is_uppercase();
        if at_word_start && chars[i].is_uppercase() {
            for (from, to) in ACRONYM_SPELLINGS {
                let from: Vec<char> = from.chars().collect();
                if chars[i..].starts_with(&from) {
                    out.push_str(to);
                    i += from.len();
                    continue 'outer;
                }
            }
            // Plural acronym: two or more capitals, then an `s` that ends the
            // word.
            let run = chars[i..].iter().take_while(|c| c.is_uppercase()).count();
            let plural_s = chars.get(i + run) == Some(&'s')
                && !chars.get(i + run + 1).is_some_and(|c| c.is_lowercase());
            if run >= 2 && plural_s {
                out.push(chars[i]);
                out.extend(chars[i + 1..i + run].iter().flat_map(|c| c.to_lowercase()));
                i += run;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
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

/// Format a union case value: returns (`case_name`, `case_value_expr`).
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
