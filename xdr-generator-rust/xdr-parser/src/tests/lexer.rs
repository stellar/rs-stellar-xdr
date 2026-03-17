use crate::lexer::{IntBase, Lexer, Token};

#[test]
fn test_ifdef_tokens() {
    let input = "#ifdef FEATURE_X\nstruct Foo {};\n#endif";
    let lexer = Lexer::new(input);
    let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
    let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
    assert_eq!(
        tokens,
        vec![
            Token::IfDef("FEATURE_X".into()),
            Token::Struct,
            Token::Ident("Foo".into()),
            Token::LBrace,
            Token::RBrace,
            Token::Semi,
            Token::EndIf,
            Token::Eof,
        ]
    );
}

#[test]
fn test_ifndef_token() {
    let input = "#ifndef MY_FEATURE";
    let lexer = Lexer::new(input);
    let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
    let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
    assert_eq!(
        tokens,
        vec![Token::IfNDef("MY_FEATURE".into()), Token::Eof,]
    );
}

#[test]
fn test_elif_token() {
    let input = "#elif OTHER_FEATURE";
    let lexer = Lexer::new(input);
    let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
    let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
    assert_eq!(
        tokens,
        vec![Token::Elif("OTHER_FEATURE".into()), Token::Eof,]
    );
}

#[test]
fn test_else_endif_tokens() {
    let input = "#else\n#endif";
    let lexer = Lexer::new(input);
    let (spanned_tokens, _) = lexer.tokenize_with_spans().unwrap();
    let tokens: Vec<Token> = spanned_tokens.into_iter().map(|st| st.token).collect();
    assert_eq!(tokens, vec![Token::Else, Token::EndIf, Token::Eof,]);
}

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
