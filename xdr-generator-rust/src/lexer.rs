//! XDR lexer (tokenizer) using Logos.

use logos::{Logos, SpannedIter};
use thiserror::Error;

/// The base (radix) of an integer literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntBase {
    Decimal,
    Hexadecimal,
}

fn parse_hex(lex: &logos::Lexer<Token>) -> Option<(i64, IntBase)> {
    let slice = lex.slice();
    i64::from_str_radix(&slice[2..], 16)
        .ok()
        .map(|v| (v, IntBase::Hexadecimal))
}

fn parse_decimal(lex: &logos::Lexer<Token>) -> Option<(i64, IntBase)> {
    lex.slice().parse().ok().map(|v| (v, IntBase::Decimal))
}

/// Token type for XDR lexing.
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\r\f]+")]  // Skip whitespace
#[logos(skip r"//[^\n]*")]      // Skip line comments
#[logos(skip r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")] // Skip block comments
#[logos(skip r"%[^\n]*\n?")]    // Skip preprocessor directives
pub enum Token {
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

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(std::string::String),

    // Integer literals - hex must have higher priority than decimal
    #[regex(r"0x[0-9a-fA-F]+", parse_hex, priority = 2)]
    #[regex(r"-?[0-9]+", parse_decimal, priority = 1)]
    IntLiteral((i64, IntBase)),

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

    // End of file (not produced by Logos, added manually)
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
    #[error("lexer error at position {0}")]
    LexerError(usize),
}

/// The main lexer for tokenizing XDR source text.
pub struct Lexer<'a> {
    source: &'a str,
    inner: SpannedIter<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input,
            inner: Token::lexer(input).spanned(),
        }
    }

    /// Tokenize with span information for each token.
    pub fn tokenize_with_spans(self) -> Result<(Vec<SpannedToken>, std::string::String), LexError> {
        let source = self.source.to_string();
        let source_len = self.source.len();
        let mut tokens = Vec::new();

        for (result, span) in self.inner {
            let token = match result {
                Ok(t) => t,
                Err(()) => {
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
                Token::IntLiteral((256, IntBase::Hexadecimal)),
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
                Token::IntLiteral((-1, IntBase::Decimal)),
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
