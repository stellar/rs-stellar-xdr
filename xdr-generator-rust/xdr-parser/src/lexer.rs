//! XDR lexer (tokenizer) using Logos.

use logos::{Logos, SpannedIter};
use thiserror::Error;

/// Token type for XDR lexing.
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\r\f]+")] // Skip whitespace
#[logos(skip r"//[^\n]*")] // Skip line comments
#[logos(skip r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")] // Skip block comments
#[logos(skip r"%[^\n]*\n?")] // Skip preprocessor directives
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

    // Preprocessor conditionals
    #[regex(r"#ifdef[ \t]+([a-zA-Z_][a-zA-Z0-9_]*)", parse_directive_ident)]
    IfDef(std::string::String),
    #[regex(r"#ifndef[ \t]+([a-zA-Z_][a-zA-Z0-9_]*)", parse_directive_ident)]
    IfNDef(std::string::String),
    #[regex(r"#elif[ \t]+([a-zA-Z_][a-zA-Z0-9_]*)", parse_directive_ident)]
    Elif(std::string::String),
    #[regex(r"#else")]
    Else,
    #[regex(r"#endif")]
    EndIf,

    // End of file (not produced by Logos, added manually)
    Eof,
}

/// The base (radix) of an integer literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntBase {
    Decimal,
    Hexadecimal,
}

fn parse_directive_ident(lex: &logos::Lexer<Token>) -> Option<std::string::String> {
    // The regex captures "#ifdef IDENT", "#ifndef IDENT", or "#elif IDENT".
    // Extract the identifier (last whitespace-separated token).
    let slice = lex.slice();
    let ident = slice.rsplit(|c: char| c.is_ascii_whitespace()).next()?;
    if ident.is_empty() {
        None
    } else {
        Some(ident.to_string())
    }
}

fn parse_hex(lex: &logos::Lexer<Token>) -> Option<(i64, IntBase)> {
    let slice = lex.slice();
    // Parse as u64 first to handle the full range of hex values (e.g., 0xFFFFFFFFFFFFFFFF),
    // then reinterpret as i64. This preserves the bit pattern for large unsigned values.
    u64::from_str_radix(&slice[2..], 16)
        .ok()
        .map(|v| (v as i64, IntBase::Hexadecimal))
}

fn parse_decimal(lex: &logos::Lexer<Token>) -> Option<(i64, IntBase)> {
    lex.slice().parse().ok().map(|v| (v, IntBase::Decimal))
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
