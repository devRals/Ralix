use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub type Literal = Rc<str>;

#[derive(Debug, PartialEq, Default, Clone)]
pub enum Token {
    #[default]
    Unknown,
    Illegal(Literal),
    Ident(Literal),
    #[allow(clippy::upper_case_acronyms)]
    EOF,
    /// "\n"
    NewLine,
    /// "->"
    ThinArrow,
    /// "=>"
    FatArrow,

    /// Example: "hello world", "uwu :3", "meow \nowo"
    String(Literal),
    /// Example: 'a', '1', '\n'
    Char(Literal),
    /// Example: 2, 3, 5, 7, 10, 0
    Int(Literal),
    /// Example: 3.141, 20.20, NaN, 0.0
    Float(Literal),

    /// "="
    Assign,
    /// "!"
    Bang,
    /// "|"
    Pipe,
    /// "&"
    Ampersant,
    /// "#"
    Hash,
    /// "@"
    WhoKnowsWhatThisIs,
    /// ?
    QuestionMark,

    /// "=="
    Equal,
    /// "!="
    NotEqual,

    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "/"
    Slash,
    /// "*"
    Asterisk,
    /// "%"
    InAHundred,

    /// "++"
    Increase,
    /// "--"
    Decrease,

    /// "<"
    LessThan,
    /// ">"
    GreaterThan,
    /// "<="
    LessEqual,
    /// ">="
    GreatEqual,

    /// ","
    Comma,
    /// "."
    Notation,
    /// ".."
    TwoDots,
    /// ";"
    SemiColon,
    /// ":"
    Colon,
    /// "("
    LParen,
    /// ")"
    RParen,
    /// "{"
    LBrace,
    /// "}"
    RBrace,
    /// "["
    LBracket,
    /// "]"
    RBracket,
    /// "::"
    Namespace,

    // # Keywords
    Null,
    If,
    Else,
    Function,
    Let,
    Const,
    Return,
    True,
    False,
    While,
    For,
    Continue,
    Break,
    Match,
    Class,
    Static,
    New,
    Interface,
    Public,
    Import,
    Not,
    And,
    Or,
    Mutable,
    Copy,

    // Type Keywords
    /// "int"
    TyInt,
    /// "float"
    TyFloat,
    /// "str"
    TyString,
    /// "char"
    TyChar,
    Bool,
    Dyn,
}
impl Token {
    pub fn keyword(keyword: &str) -> Token {
        match keyword {
            "null" => Token::Null,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "const" => Token::Const,
            "let" => Token::Let,
            "while" => Token::While,
            "for" => Token::For,
            "return" => Token::Return,
            "continue" => Token::Continue,
            "break" => Token::Break,
            "match" => Token::Match,
            "class" => Token::Class,
            "static" => Token::Static,
            "interface" => Token::Interface,
            "new" => Token::New,
            "public" => Token::Public,
            "import" => Token::Import,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "int" => Token::TyInt,
            "float" => Token::TyFloat,
            "str" => Token::TyString,
            "char" => Token::TyChar,
            "bool" => Token::Bool,
            "dyn" => Token::Dyn,
            "mut" => Token::Mutable,
            "copy" => Token::Copy,
            lit => Token::Ident(Literal::from(lit)),
        }
    }

    pub fn literal(&self) -> Literal {
        use Token as T;
        match self {
            T::Ident(lit) => lit.clone(),
            T::Illegal(lit) => lit.clone(),
            T::String(lit) => lit.clone(),
            T::Char(lit) => lit.clone(),
            T::Int(lit) => lit.clone(),
            T::Float(lit) => lit.clone(),
            T::True => Literal::from("true"),
            T::False => Literal::from("true"),
            t => Literal::from(t.to_string()),
        }
        .clone()
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token as T;
        f.write_str(match self {
            T::Ident(_) => "identifier",
            T::Illegal(lit) => lit,
            T::Unknown => "unknown",
            T::EOF => "EOF",
            T::TyInt | T::Int(_) => "int",
            T::TyFloat | T::Float(_) => "float",
            T::Bool | T::True | T::False => "bool",
            T::TyChar | T::Char(_) => "char",
            T::TyString | T::String(_) => "str",

            T::ThinArrow => "->",
            T::FatArrow => "=>",

            T::NewLine => "\n",
            T::Assign => "=",
            T::Bang => "!",
            T::Pipe => "|",
            T::Ampersant => "&",
            T::Hash => "#",
            T::WhoKnowsWhatThisIs => "@",
            T::QuestionMark => "?",

            T::Equal => "==",
            T::NotEqual => "!=",
            T::Plus => "+",
            T::Minus => "-",
            T::Slash => "/",
            T::InAHundred => "%",

            T::Asterisk => "*",
            T::Increase => "++",
            T::Decrease => "--",
            T::LessThan => "<",
            T::GreaterThan => ">",
            T::LessEqual => "<=",
            T::GreatEqual => ">=",
            T::Comma => ",",
            T::Notation => ".",
            T::TwoDots => "..",
            T::SemiColon => ";",
            T::Colon => ":",
            T::LParen => "(",
            T::RParen => ")",
            T::LBrace => "{",
            T::RBrace => "}",
            T::LBracket => "[",
            T::RBracket => "]",
            T::Namespace => "::",

            T::Null => "null",
            T::If => "if",
            T::Else => "else",
            T::Function => "fn",
            T::Let => "let",
            T::Const => "const",
            T::Return => "return",
            T::While => "while",
            T::For => "for",
            T::Continue => "continue",
            T::Break => "break",
            T::Match => "match",
            T::Class => "class",
            T::Static => "static",
            T::New => "new",
            T::Interface => "interface",
            T::Public => "public",
            T::Import => "import",
            T::Not => "not",
            T::And => "and",
            T::Or => "or",
            T::Dyn => "dyn",
            T::Mutable => "mut",
            T::Copy => "copy",
        })
    }
}
