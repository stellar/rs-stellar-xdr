use std::collections::HashSet;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;

use clap::Args;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("line {line}: unmatched #endif")]
    UnmatchedEndif { line: usize },
    #[error("line {line}: unmatched #else")]
    UnmatchedElse { line: usize },
    #[error("line {line}: unmatched #elif")]
    UnmatchedElif { line: usize },
    #[error("line {line}: unclosed #ifdef/#ifndef")]
    UnmatchedIfdef { line: usize },
    #[error("line {line}: duplicate #else")]
    DuplicateElse { line: usize },
    #[error("line {line}: #elif after #else")]
    ElifAfterElse { line: usize },
    #[error("line {line}: {directive} missing symbol")]
    MissingSymbol { line: usize, directive: String },
    #[error("line {line}: invalid {directive} syntax")]
    InvalidDirectiveSyntax { line: usize, directive: String },
    #[error("error reading file: {0}")]
    ReadFile(std::io::Error),
    #[error("error writing output: {0}")]
    WriteOutput(std::io::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    /// XDR .x file to preprocess, or stdin if omitted
    #[arg()]
    pub input: Option<PathBuf>,

    /// Comma-separated list of features/symbols to define
    #[arg(long)]
    pub features: Option<String>,
}

#[allow(clippy::struct_excessive_bools)]
struct IfBlock {
    active: bool,
    parent_active: bool,
    any_branch_taken: bool,
    else_seen: bool,
    start_line: usize,
}

fn is_comment_or_empty(rest: &str) -> bool {
    let rest = rest.trim_start();
    rest.is_empty() || rest.starts_with("//") || rest.starts_with("/*")
}

fn parse_symbol_directive<'a>(line: &'a str, directive: &str) -> Result<Option<&'a str>, ()> {
    let Some(rest) = line.strip_prefix(directive) else {
        return Ok(None);
    };

    if rest.is_empty() {
        return Err(());
    }

    let mut chars = rest.chars();
    let Some(first) = chars.next() else {
        return Err(());
    };
    if !first.is_whitespace() {
        return Err(());
    }

    let after_ws = rest.trim_start();
    if after_ws.is_empty() || after_ws.starts_with("//") || after_ws.starts_with("/*") {
        return Err(());
    }

    let symbol_end = after_ws.find(char::is_whitespace).unwrap_or(after_ws.len());
    let symbol = &after_ws[..symbol_end];
    if symbol.is_empty() {
        return Err(());
    }

    let trailing = &after_ws[symbol_end..];
    if !is_comment_or_empty(trailing) {
        return Err(());
    }

    Ok(Some(symbol))
}

fn is_flagless_directive(line: &str, directive: &str) -> Result<bool, ()> {
    let Some(rest) = line.strip_prefix(directive) else {
        return Ok(false);
    };

    if rest.is_empty() {
        return Ok(true);
    }

    let first = rest.chars().next().unwrap_or(' ');
    if !first.is_whitespace() {
        return Err(());
    }

    if is_comment_or_empty(rest) {
        Ok(true)
    } else {
        Err(())
    }
}

fn symbol_directive_err(trimmed: &str, directive: &str, line: usize) -> Error {
    if trimmed == directive {
        Error::MissingSymbol {
            line,
            directive: directive.to_string(),
        }
    } else {
        Error::InvalidDirectiveSyntax {
            line,
            directive: directive.to_string(),
        }
    }
}

fn flagless_directive_err(directive: &str, line: usize) -> Error {
    Error::InvalidDirectiveSyntax {
        line,
        directive: directive.to_string(),
    }
}

/// Preprocess XDR `.x` source by evaluating
/// `#ifdef`/`#ifndef`/`#elif`/`#else`/`#endif` directives against the given
/// set of defined feature symbols.
///
/// # Errors
///
/// Returns an error for malformed or unbalanced preprocessor directives.
///
/// # Panics
///
/// Panics if `parse_symbol_directive` returns `Ok(None)` after a
/// `starts_with` guard, which cannot happen in practice.
pub fn preprocess(input: &str, features: &[&str]) -> Result<String, Error> {
    let mut output = String::new();
    let mut stack: Vec<IfBlock> = Vec::new();
    let feature_set: HashSet<&str> = features.iter().copied().collect();

    for (i, raw_line) in input.split_inclusive('\n').enumerate() {
        let line_num = i + 1;
        let trimmed = raw_line.strip_suffix('\n').unwrap_or(raw_line).trim();
        process_line(
            trimmed,
            line_num,
            raw_line,
            &mut stack,
            &feature_set,
            &mut output,
        )?;
    }

    if let Some(block) = stack.last() {
        return Err(Error::UnmatchedIfdef {
            line: block.start_line,
        });
    }

    Ok(output)
}

#[allow(clippy::too_many_lines)]
fn process_line(
    trimmed: &str,
    line_num: usize,
    raw_line: &str,
    stack: &mut Vec<IfBlock>,
    feature_set: &HashSet<&str>,
    output: &mut String,
) -> Result<(), Error> {
    if trimmed.starts_with("#ifdef") {
        let symbol = parse_symbol_directive(trimmed, "#ifdef")
            .map_err(|()| symbol_directive_err(trimmed, "#ifdef", line_num))?
            .expect("starts_with guard ensures strip_prefix succeeds");
        let parent_active = stack.last().is_none_or(|b| b.active);
        let active = parent_active && feature_set.contains(symbol);
        stack.push(IfBlock {
            active,
            parent_active,
            any_branch_taken: active,
            else_seen: false,
            start_line: line_num,
        });
    } else if trimmed.starts_with("#ifndef") {
        let symbol = parse_symbol_directive(trimmed, "#ifndef")
            .map_err(|()| symbol_directive_err(trimmed, "#ifndef", line_num))?
            .expect("starts_with guard ensures strip_prefix succeeds");
        let parent_active = stack.last().is_none_or(|b| b.active);
        let active = parent_active && !feature_set.contains(symbol);
        stack.push(IfBlock {
            active,
            parent_active,
            any_branch_taken: active,
            else_seen: false,
            start_line: line_num,
        });
    } else if trimmed.starts_with("#elif") {
        let symbol = parse_symbol_directive(trimmed, "#elif")
            .map_err(|()| symbol_directive_err(trimmed, "#elif", line_num))?
            .expect("starts_with guard ensures strip_prefix succeeds");
        let block = stack
            .last_mut()
            .ok_or(Error::UnmatchedElif { line: line_num })?;
        if block.else_seen {
            return Err(Error::ElifAfterElse { line: line_num });
        }
        let newly_active =
            block.parent_active && !block.any_branch_taken && feature_set.contains(symbol);
        block.active = newly_active;
        block.any_branch_taken = block.any_branch_taken || newly_active;
    } else if trimmed.starts_with("#else") {
        is_flagless_directive(trimmed, "#else")
            .map_err(|()| flagless_directive_err("#else", line_num))?;
        let block = stack
            .last_mut()
            .ok_or(Error::UnmatchedElse { line: line_num })?;
        if block.else_seen {
            return Err(Error::DuplicateElse { line: line_num });
        }
        block.else_seen = true;
        block.active = block.parent_active && !block.any_branch_taken;
    } else if trimmed.starts_with("#endif") {
        is_flagless_directive(trimmed, "#endif")
            .map_err(|()| flagless_directive_err("#endif", line_num))?;
        if stack.pop().is_none() {
            return Err(Error::UnmatchedEndif { line: line_num });
        }
    } else if stack.last().is_none_or(|b| b.active) {
        output.push_str(raw_line);
    }
    Ok(())
}

impl Cmd {
    /// Run the CLIs preprocess command.
    ///
    /// ## Errors
    ///
    /// If the command is configured with state that is invalid.
    pub fn run(&self) -> Result<(), Error> {
        let features: Vec<&str> = self
            .features
            .as_deref()
            .map(|s| {
                s.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        let mut content = String::new();
        let mut reader: Box<dyn Read> = match &self.input {
            Some(path) => Box::new(File::open(path).map_err(Error::ReadFile)?),
            None => Box::new(stdin()),
        };
        reader
            .read_to_string(&mut content)
            .map_err(Error::ReadFile)?;
        let result = preprocess(&content, &features)?;
        match stdout().write_all(result.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
            Err(e) => Err(Error::WriteOutput(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_directives_passthrough() {
        let input = "line one\nline two\nline three\n";
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "line one\nline two\nline three\n");
    }

    #[test]
    fn test_ifdef_defined_includes_content() {
        let input = "\
#ifdef FEAT_A
included
#endif
";
        let result = preprocess(input, &["FEAT_A"]).unwrap();
        assert_eq!(result, "included\n");
    }

    #[test]
    fn test_ifdef_undefined_excludes_content() {
        let input = "\
#ifdef FEAT_A
excluded
#endif
";
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_ifdef_else_defined() {
        let input = "\
#ifdef FEAT_A
yes
#else
no
#endif
";
        let result = preprocess(input, &["FEAT_A"]).unwrap();
        assert_eq!(result, "yes\n");
    }

    #[test]
    fn test_ifdef_else_undefined() {
        let input = "\
#ifdef FEAT_A
yes
#else
no
#endif
";
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "no\n");
    }

    #[test]
    fn test_ifndef_defined_excludes() {
        let input = "\
#ifndef FEAT_A
included
#endif
";
        let result = preprocess(input, &["FEAT_A"]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_ifndef_undefined_includes() {
        let input = "\
#ifndef FEAT_A
included
#endif
";
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "included\n");
    }

    #[test]
    fn test_nested_ifdefs() {
        let input = "\
#ifdef OUTER
outer_content
#ifdef INNER
inner_content
#endif
after_inner
#endif
";
        // Both defined
        let result = preprocess(input, &["OUTER", "INNER"]).unwrap();
        assert_eq!(result, "outer_content\ninner_content\nafter_inner\n");

        // Only outer defined
        let result = preprocess(input, &["OUTER"]).unwrap();
        assert_eq!(result, "outer_content\nafter_inner\n");

        // Neither defined
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_nested_ifdef_parent_inactive() {
        let input = "\
#ifdef OUTER
#ifdef INNER
should_not_appear
#endif
#endif
";
        // Inner defined but outer not — inner should still be excluded
        let result = preprocess(input, &["INNER"]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_error_unmatched_endif() {
        let input = "#endif\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(err, Error::UnmatchedEndif { line: 1 }));
    }

    #[test]
    fn test_error_unclosed_ifdef() {
        let input = "#ifdef FEAT_A\ncontent\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(err, Error::UnmatchedIfdef { line: 1 }));
    }

    #[test]
    fn test_error_duplicate_else() {
        let input = "\
#ifdef FEAT_A
a
#else
b
#else
c
#endif
";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(err, Error::DuplicateElse { line: 5 }));
    }

    #[test]
    fn test_error_ifdef_missing_symbol() {
        let input = "#ifdef\n#endif\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(
            err,
            Error::MissingSymbol {
                line: 1,
                directive: _
            }
        ));
    }

    #[test]
    fn test_error_ifndef_missing_symbol() {
        let input = "#ifndef\n#endif\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(
            err,
            Error::MissingSymbol {
                line: 1,
                directive: _
            }
        ));
    }

    #[test]
    fn test_realistic_xdr_pattern() {
        let input = "\
enum SCValType
{
    SCV_BOOL = 0,
    SCV_VOID = 1,
    SCV_ERROR = 2,
#ifdef XDR_SPARSE_MAP
    SCV_SPARSE_MAP = 22,
#endif
    SCV_U32 = 4
};

#ifndef XDR_SPARSE_MAP
struct FallbackDef {
    int x;
};
#endif
";
        // With feature defined
        let result = preprocess(input, &["XDR_SPARSE_MAP"]).unwrap();
        assert_eq!(
            result,
            "\
enum SCValType
{
    SCV_BOOL = 0,
    SCV_VOID = 1,
    SCV_ERROR = 2,
    SCV_SPARSE_MAP = 22,
    SCV_U32 = 4
};

"
        );

        // Without feature defined
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(
            result,
            "\
enum SCValType
{
    SCV_BOOL = 0,
    SCV_VOID = 1,
    SCV_ERROR = 2,
    SCV_U32 = 4
};

struct FallbackDef {
    int x;
};
"
        );
    }

    #[test]
    fn test_multiple_features() {
        let input = "\
#ifdef FEAT_A
a
#endif
#ifdef FEAT_B
b
#endif
";
        let result = preprocess(input, &["FEAT_A", "FEAT_B"]).unwrap();
        assert_eq!(result, "a\nb\n");

        let result = preprocess(input, &["FEAT_A"]).unwrap();
        assert_eq!(result, "a\n");
    }

    #[test]
    fn test_empty_input() {
        let result = preprocess("", &[]).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_preserves_indentation() {
        let input = "\
#ifdef FEAT
    indented line
        deeply indented
#endif
";
        let result = preprocess(input, &["FEAT"]).unwrap();
        assert_eq!(result, "    indented line\n        deeply indented\n");
    }

    #[test]
    fn test_error_invalid_ifdef_syntax_missing_space() {
        let input = "#ifdefFEAT_A\ncontent\n#endif\n";
        let err = preprocess(input, &["FEAT_A"]).unwrap_err();
        assert!(matches!(
            err,
            Error::InvalidDirectiveSyntax {
                line: 1,
                directive: _
            }
        ));
    }

    #[test]
    fn test_error_invalid_endif_syntax_missing_space() {
        let input = "#ifdef FEAT_A\ncontent\n#endifX\n";
        let err = preprocess(input, &["FEAT_A"]).unwrap_err();
        assert!(matches!(
            err,
            Error::InvalidDirectiveSyntax {
                line: 3,
                directive: _
            }
        ));
    }

    #[test]
    fn test_else_with_trailing_comment() {
        let input = "\
#ifdef FEAT_A
yes
#else // fallback
no
#endif
";
        let result = preprocess(input, &["FEAT_A"]).unwrap();
        assert_eq!(result, "yes\n");

        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "no\n");
    }

    #[test]
    fn test_preserves_missing_final_newline() {
        let result = preprocess("A", &[]).unwrap();
        assert_eq!(result, "A");
    }

    #[test]
    fn test_preserves_crlf_newlines() {
        let input = "A\r\nB\r\n";
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result.as_bytes(), input.as_bytes());
    }

    #[test]
    fn test_elif_first_branch() {
        let input = "\
#ifdef A
a
#elif B
b
#elif C
c
#else
d
#endif
";
        let result = preprocess(input, &["A", "B", "C"]).unwrap();
        assert_eq!(result, "a\n");
    }

    #[test]
    fn test_elif_second_branch() {
        let input = "\
#ifdef A
a
#elif B
b
#elif C
c
#else
d
#endif
";
        let result = preprocess(input, &["B"]).unwrap();
        assert_eq!(result, "b\n");
    }

    #[test]
    fn test_elif_third_branch() {
        let input = "\
#ifdef A
a
#elif B
b
#elif C
c
#else
d
#endif
";
        let result = preprocess(input, &["C"]).unwrap();
        assert_eq!(result, "c\n");
    }

    #[test]
    fn test_elif_else_fallback() {
        let input = "\
#ifdef A
a
#elif B
b
#else
fallback
#endif
";
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "fallback\n");
    }

    #[test]
    fn test_elif_no_else() {
        let input = "\
#ifdef A
a
#elif B
b
#endif
";
        // Neither defined
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "");

        // Only B defined
        let result = preprocess(input, &["B"]).unwrap();
        assert_eq!(result, "b\n");
    }

    #[test]
    fn test_elif_nested() {
        let input = "\
#ifdef OUTER
#ifdef A
a
#elif B
b
#endif
#endif
";
        // Outer inactive — inner elif should not activate
        let result = preprocess(input, &["B"]).unwrap();
        assert_eq!(result, "");

        // Both outer and B active
        let result = preprocess(input, &["OUTER", "B"]).unwrap();
        assert_eq!(result, "b\n");
    }

    #[test]
    fn test_error_elif_after_else() {
        let input = "\
#ifdef A
a
#else
b
#elif C
c
#endif
";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(err, Error::ElifAfterElse { line: 5 }));
    }

    #[test]
    fn test_error_elif_missing_symbol() {
        let input = "#ifdef A\n#elif\n#endif\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(
            err,
            Error::MissingSymbol {
                line: 2,
                directive: _
            }
        ));
    }

    #[test]
    fn test_error_elif_without_ifdef() {
        let input = "#elif A\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(err, Error::UnmatchedElif { line: 1 }));
    }

    #[test]
    fn test_error_ifdef_comment_as_symbol() {
        let input = "#ifdef // comment\n#endif\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(
            err,
            Error::InvalidDirectiveSyntax {
                line: 1,
                directive: _
            }
        ));
    }

    #[test]
    fn test_error_endif_trailing_tokens() {
        let input = "#ifdef FEAT_A\ncontent\n#endif EXTRA\n";
        let err = preprocess(input, &["FEAT_A"]).unwrap_err();
        assert!(matches!(
            err,
            Error::InvalidDirectiveSyntax {
                line: 3,
                directive: _
            }
        ));
    }

    #[test]
    fn test_error_else_trailing_tokens() {
        let input = "#ifdef FEAT_A\na\n#else EXTRA\nb\n#endif\n";
        let err = preprocess(input, &[]).unwrap_err();
        assert!(matches!(
            err,
            Error::InvalidDirectiveSyntax {
                line: 3,
                directive: _
            }
        ));
    }

    #[test]
    fn test_ifndef_with_elif() {
        let input = "\
#ifndef A
a
#elif B
b
#else
fallback
#endif
";
        // A not defined → ifndef branch taken, elif/else skipped
        let result = preprocess(input, &[]).unwrap();
        assert_eq!(result, "a\n");

        // A not defined, B defined → ifndef branch taken, elif/else skipped
        let result = preprocess(input, &["B"]).unwrap();
        assert_eq!(result, "a\n");

        // A defined, B defined → ifndef false, elif B taken
        let result = preprocess(input, &["A", "B"]).unwrap();
        assert_eq!(result, "b\n");

        // A defined, B not defined → ifndef false, elif false, else taken
        let result = preprocess(input, &["A"]).unwrap();
        assert_eq!(result, "fallback\n");
    }
}
