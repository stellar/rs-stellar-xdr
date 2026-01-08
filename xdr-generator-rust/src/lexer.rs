//! XDR lexer (tokenizer).

use std::iter::Peekable;
use std::str::Chars;

use thiserror::Error;

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
    IntLiteral(i64),

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

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("unterminated block comment")]
    UnterminatedComment,
    #[error("invalid integer literal: {0}")]
    InvalidInt(std::string::String),
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    /// Track position for error reporting
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.input.next()?;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(&c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), LexError> {
        loop {
            match self.advance() {
                None => return Err(LexError::UnterminatedComment),
                Some('*') => {
                    if self.peek() == Some(&'/') {
                        self.advance();
                        return Ok(());
                    }
                }
                Some(_) => {}
            }
        }
    }

    fn read_ident(&mut self, first: char) -> std::string::String {
        let mut s = std::string::String::new();
        s.push(first);
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn read_number(&mut self, first: char) -> Result<i64, LexError> {
        let mut s = std::string::String::new();
        s.push(first);

        // Check for hex
        if first == '0' && self.peek() == Some(&'x') {
            s.push('x');
            self.advance();
            while let Some(&c) = self.peek() {
                if c.is_ascii_hexdigit() {
                    s.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            return i64::from_str_radix(&s[2..], 16).map_err(|_| LexError::InvalidInt(s));
        }

        // Decimal
        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        s.parse().map_err(|_| LexError::InvalidInt(s))
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        loop {
            self.skip_whitespace();

            match self.peek() {
                None => return Ok(Token::Eof),
                Some(&'/') => {
                    self.advance();
                    match self.peek() {
                        Some(&'/') => {
                            self.advance();
                            self.skip_line_comment();
                            continue;
                        }
                        Some(&'*') => {
                            self.advance();
                            self.skip_block_comment()?;
                            continue;
                        }
                        _ => return Err(LexError::UnexpectedChar('/')),
                    }
                }
                _ => break,
            }
        }

        let c = match self.advance() {
            None => return Ok(Token::Eof),
            Some(c) => c,
        };

        match c {
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            '<' => Ok(Token::LAngle),
            '>' => Ok(Token::RAngle),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            ';' => Ok(Token::Semi),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '*' => Ok(Token::Star),
            '=' => Ok(Token::Eq),

            '-' => {
                // Negative number
                match self.peek() {
                    Some(&c) if c.is_ascii_digit() => {
                        let first = self.advance().unwrap();
                        let n = self.read_number(first)?;
                        Ok(Token::IntLiteral(-n))
                    }
                    _ => Err(LexError::UnexpectedChar('-')),
                }
            }

            c if c.is_ascii_digit() => {
                let n = self.read_number(c)?;
                Ok(Token::IntLiteral(n))
            }

            c if c.is_alphabetic() || c == '_' => {
                let ident = self.read_ident(c);
                let token = match ident.as_str() {
                    "struct" => Token::Struct,
                    "enum" => Token::Enum,
                    "union" => Token::Union,
                    "typedef" => Token::Typedef,
                    "const" => Token::Const,
                    "namespace" => Token::Namespace,
                    "switch" => Token::Switch,
                    "case" => Token::Case,
                    "default" => Token::Default,
                    "void" => Token::Void,
                    "unsigned" => Token::Unsigned,
                    "int" => Token::Int,
                    "hyper" => Token::Hyper,
                    "float" => Token::Float,
                    "double" => Token::Double,
                    "bool" => Token::Bool,
                    "opaque" => Token::Opaque,
                    "string" => Token::String,
                    _ => Token::Ident(ident),
                };
                Ok(token)
            }

            _ => Err(LexError::UnexpectedChar(c)),
        }
    }

    /// Tokenize the entire input.
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "struct Foo { int x; };";
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
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
        let tokens = lexer.tokenize().unwrap();
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
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Ident("KEY_TYPE_MUXED_ED25519".into()),
                Token::Eq,
                Token::IntLiteral(256),
                Token::Eof,
            ]
        );
    }
}
