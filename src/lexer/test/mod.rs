use crate::{Lexer, Literal, Token};

#[test]
fn test_lexer() {
    use Token::*;
    let src = r#"
        //  Values and Logic Operators
        "meow" 'a' = ! | & # @ ? == !=
        + - * / ++ -- < > <= >=

        //  Symbols
        , . .. ; : ( ) { } [ ] ::

        //  Keywords
        null if else fn const return
        true false while for continue
        break match struct interface
        export not and or
    "#;
    let expected_tokens = [
        String(Literal::from("meow")),
        Char(Literal::from("a")),
        Assign,
        Bang,
        Pipe,
        Ampersant,
        Hash,
        AtSign,
        QuestionMark,
        Equal,
        NotEqual,
        Plus,
        Minus,
        Asterisk,
        Slash,
        Increase,
        Decrease,
        LessThan,
        GreaterThan,
        LessEqual,
        GreatEqual,
        Comma,
        Dot,
        TwoDots,
        SemiColon,
        Colon,
        LParen,
        RParen,
        LBrace,
        RBrace,
        LBracket,
        RBracket,
        Namespace,
        Null,
        If,
        Else,
        Function,
        Const,
        Return,
        True,
        False,
        While,
        For,
        Continue,
        Break,
        Match,
        Struct,
        Interface,
        Export,
        Not,
        And,
        Or,
    ];

    let lexer = Lexer::new(src);

    for (t1, t2) in lexer.zip(expected_tokens) {
        assert_eq!(t1, t2)
    }
}
