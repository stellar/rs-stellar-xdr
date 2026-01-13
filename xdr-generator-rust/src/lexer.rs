//! XDR lexer (tokenizer) using Logos.

use logos::{Logos, SpannedIter};
use thiserror::Error;

/// Internal token type used by Logos for lexing.
/// This handles the raw tokenization including keywords.
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\r\f]+")]  // Skip whitespace
#[logos(skip r"//[^\n]*")]      // Skip line comments
#[logos(skip r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")] // Skip block comments
#[logos(skip r"%[^\n]*\n?")]    // Skip preprocessor directives
enum RawToken {
    // Keywords
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("union")]
    Union,
    #[token("typedef")]
    Typedef,
    #[token("const")]
    Const,
    #[token("namespace")]
    Namespace,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("void")]
    Void,
    #[token("unsigned")]
    Unsigned,
    #[token("int")]
    Int,
    #[token("hyper")]
    Hyper,
    #[token("float")]
    Float,
    #[token("double")]
    Double,
    #[token("bool")]
    Bool,
    #[token("opaque")]
    Opaque,
    #[token("string")]
    String,

    // Identifiers (must come after keywords due to priority)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(std::string::String),

    // Integer literals - hex (must come before decimal for priority)
    #[regex(r"0x[0-9a-fA-F]+", parse_hex)]
    HexLiteral(i64),

    // Integer literals - decimal (including negative)
    #[regex(r"-?[0-9]+", parse_decimal)]
    DecLiteral(i64),

    // Symbols
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(";")]
    Semi,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("*")]
    Star,
    #[token("=")]
    Eq,
}

fn parse_hex(lex: &logos::Lexer<RawToken>) -> Option<i64> {
    let slice = lex.slice();
    i64::from_str_radix(&slice[2..], 16).ok()
}

fn parse_decimal(lex: &logos::Lexer<RawToken>) -> Option<i64> {
    lex.slice().parse().ok()
}

/// The public token type matching the original lexer's interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Keywords
    Struct,
    Enum,
    Union,
    Typedef,
    Const,
    Namespace,
    Switch,
    Case,
    Default,
    Void,
    Unsigned,
    Int,
    Hyper,
    Float,
    Double,
    Bool,
    Opaque,
    String,

    // Identifiers and literals
    Ident(std::string::String),
    /// Integer literal with value and whether it was in hex format
    IntLiteral { value: i64, is_hex: bool },

    // Symbols
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    LAngle,   // <
    RAngle,   // >
    LParen,   // (
    RParen,   // )
    Semi,     // ;
    Colon,    // :
    Comma,    // ,
    Star,     // *
    Eq,       // =

    // End of file
    Eof,
}

/// A token with its byte span in the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken {
    pub token: Token,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("unterminated block comment")]
    UnterminatedComment,
    #[error("invalid integer literal: {0}")]
    InvalidInt(std::string::String),
    #[error("lexer error at position {0}")]
    LexerError(usize),
}

/// The main lexer for tokenizing XDR source text.
pub struct Lexer<'a> {
    source: &'a str,
    inner: SpannedIter<'a, RawToken>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input,
            inner: RawToken::lexer(input).spanned(),
        }
    }

    /// Tokenize with span information for each token.
    pub fn tokenize_with_spans(self) -> Result<(Vec<SpannedToken>, std::string::String), LexError> {
        let source = self.source.to_string();
        let source_len = self.source.len();
        let mut tokens = Vec::new();

        for (result, span) in self.inner {
            let token = match result {
                Ok(raw) => convert_token(raw),
                Err(()) => {
                    // Try to get the problematic character for a better error message
                    if let Some(c) = self.source[span.start..].chars().next() {
                        return Err(LexError::UnexpectedChar(c));
                    }
                    return Err(LexError::LexerError(span.start));
                }
            };

            tokens.push(SpannedToken {
                token,
                start: span.start,
                end: span.end,
            });
        }

        // Add EOF token
        tokens.push(SpannedToken {
            token: Token::Eof,
            start: source_len,
            end: source_len,
        });

        Ok((tokens, source))
    }
}

/// Convert a RawToken to the public Token type.
fn convert_token(raw: RawToken) -> Token {
    match raw {
        RawToken::Struct => Token::Struct,
        RawToken::Enum => Token::Enum,
        RawToken::Union => Token::Union,
        RawToken::Typedef => Token::Typedef,
        RawToken::Const => Token::Const,
        RawToken::Namespace => Token::Namespace,
        RawToken::Switch => Token::Switch,
        RawToken::Case => Token::Case,
        RawToken::Default => Token::Default,
        RawToken::Void => Token::Void,
        RawToken::Unsigned => Token::Unsigned,
        RawToken::Int => Token::Int,
        RawToken::Hyper => Token::Hyper,
        RawToken::Float => Token::Float,
        RawToken::Double => Token::Double,
        RawToken::Bool => Token::Bool,
        RawToken::Opaque => Token::Opaque,
        RawToken::String => Token::String,
        RawToken::Ident(s) => Token::Ident(s),
        RawToken::HexLiteral(v) => Token::IntLiteral {
            value: v,
            is_hex: true,
        },
        RawToken::DecLiteral(v) => Token::IntLiteral {
            value: v,
            is_hex: false,
        },
        RawToken::LBrace => Token::LBrace,
        RawToken::RBrace => Token::RBrace,
        RawToken::LBracket => Token::LBracket,
        RawToken::RBracket => Token::RBracket,
        RawToken::LAngle => Token::LAngle,
        RawToken::RAngle => Token::RAngle,
        RawToken::LParen => Token::LParen,
        RawToken::RParen => Token::RParen,
        RawToken::Semi => Token::Semi,
        RawToken::Colon => Token::Colon,
        RawToken::Comma => Token::Comma,
        RawToken::Star => Token::Star,
        RawToken::Eq => Token::Eq,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "struct Foo { int x; };";
        let lexer = Lexer::new(input);
        let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Struct,
                Token::Ident("Foo".into()),
                Token::LBrace,
                Token::Int,
                Token::Ident("x".into()),
                Token::Semi,
                Token::RBrace,
                Token::Semi,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_comments() {
        let input = r#"
        // line comment
        struct /* block comment */ Foo { };
        "#;
        let lexer = Lexer::new(input);
        let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Struct,
                Token::Ident("Foo".into()),
                Token::LBrace,
                Token::RBrace,
                Token::Semi,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_hex() {
        let input = "KEY_TYPE_MUXED_ED25519 = 0x100";
        let lexer = Lexer::new(input);
        let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("KEY_TYPE_MUXED_ED25519".into()),
                Token::Eq,
                Token::IntLiteral {
                    value: 256,
                    is_hex: true
                },
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_negative_number() {
        let input = "const FOO = -1;";
        let lexer = Lexer::new(input);
        let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Const,
                Token::Ident("FOO".into()),
                Token::Eq,
                Token::IntLiteral {
                    value: -1,
                    is_hex: false
                },
                Token::Semi,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_preprocessor() {
        let input = "%#include \"xdr.h\"\nstruct Foo {};";
        let lexer = Lexer::new(input);
        let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
        let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
        assert_eq!(
            tokens,
            vec![
                Token::Struct,
                Token::Ident("Foo".into()),
                Token::LBrace,
                Token::RBrace,
                Token::Semi,
                Token::Eof,
            ]
        );
    }
}
