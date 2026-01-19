use std::{iter::Peekable, str::Chars};

#[cfg(test)]
mod test;

pub mod token;

pub use token::*;

pub struct Lexer<'src> {
    source: Peekable<Chars<'src>>,
    current_char: Option<char>,
    peek_char: Option<char>,
}

impl<'src> Lexer<'src> {
    pub fn new(source_str: &'src str) -> Self {
        let mut source = source_str.chars().peekable();

        Self {
            current_char: source.next(),
            peek_char: source.peek().copied(),
            source,
        }
    }
}

impl Lexer<'_> {
    fn read_char(&mut self) {
        self.current_char = self.source.next();
        self.peek_char = self.source.peek().copied();
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char
            && ch.is_ascii_whitespace()
        {
            self.read_char();
        }
    }

    fn skip_comments(&mut self) {
        self.skip_whitespace();
        match self.current_char {
            Some('/') if matches!(self.peek_char, Some('/')) => {
                loop {
                    self.read_char();
                    if let Some('\n') = self.current_char {
                        break;
                    }
                }
                self.read_char(); // Consume newline
            }
            Some('/') if matches!(self.peek_char, Some('*')) => {
                loop {
                    self.read_char();

                    if matches!(self.current_char, Some('*')) && matches!(self.peek_char, Some('/'))
                    {
                        break;
                    }
                }
                self.read_char(); // Consume '*'
                self.read_char(); // Consume '/'
            }
            _ => {}
        }
        self.skip_whitespace();
    }

    fn number_token(&mut self) -> Token {
        let mut result = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char
            && (ch.is_ascii_digit() || ch == '_' || ch == '.')
        {
            if ch == '.' {
                if is_float {
                    break;
                } else {
                    is_float = true;
                }
            }
            result.push(ch);
            self.read_char();
        }

        let lit = Literal::from(result);
        if is_float {
            Token::Float(lit)
        } else {
            Token::Int(lit)
        }
    }

    fn ident_token(&mut self) -> Token {
        let mut literal = String::new();
        while let Some(current_ch) = self.current_char
            && (current_ch.is_alphanumeric() || current_ch == '_')
        {
            literal.push(current_ch);
            self.read_char();
        }

        Token::keyword(&literal)
    }

    fn string_token(&mut self) -> Token {
        let mut result = String::new();
        self.read_char(); // Consume the '"'

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.read_char();
                break;
            }

            if ch == '\\' {
                if let Some(escaped_ch) = self.read_escape() {
                    result.push(escaped_ch)
                }
            } else {
                result.push(ch)
            }

            self.read_char();
        }

        Token::String(Literal::from(result))
    }

    fn char_token(&mut self) -> Token {
        let mut result = String::new();
        self.read_char(); // Consume the '\''

        if let Some(ch) = self.current_char {
            if ch == '\\' {
                if let Some(escaped_ch) = self.read_escape() {
                    self.read_char();
                    result.push(escaped_ch);
                }
            } else {
                result.push(ch);
                self.read_char();

                if let Some(tick /* '\'' */) = self.current_char
                    && tick != '\''
                {
                    return Token::Illegal(Literal::from(result));
                }
            }
        }
        self.read_char(); // Consume the '\''
        Token::Char(Literal::from(result))
    }

    fn read_escape(&mut self) -> Option<char> {
        self.read_char(); // consume the '\\'

        match self.current_char? {
            'e' => Some('\x1b'),
            'n' => Some('\n'),
            't' => Some('\t'),
            'r' => Some('\r'),
            '\\' => Some('\\'),
            '"' => Some('"'),
            '\'' => Some('\''),
            'x' => {
                // Handle hex escape \xHH
                let mut hex_str = String::with_capacity(2);

                self.read_char(); // Move to first hex digit
                if let Some(c) = self.current_char
                    && c.is_ascii_hexdigit()
                {
                    hex_str.push(c);
                }

                self.read_char(); // Move to second hex digit
                if let Some(c) = self.current_char
                    && c.is_ascii_hexdigit()
                {
                    hex_str.push(c);
                }

                u8::from_str_radix(&hex_str, 16).ok().map(|b| b as char)
            }
            'u' => {
                // \u{1F600}
                self.read_char(); // consume first char after 'u'
                if self.current_char != Some('{') {
                    return None;
                }
                self.read_char(); // consume '{'

                let mut unicode = String::new();
                while self.current_char != Some('}') && self.current_char.is_some() {
                    unicode.push(self.current_char.unwrap());
                    self.read_char();
                }
                u32::from_str_radix(&unicode, 16)
                    .ok()
                    .and_then(char::from_u32)
            }
            other => Some(other), // Unrecognized escape, return the char itself
        }
    }

    fn parse_token(&mut self) -> Token {
        macro_rules! token {
            ( $token_ty_default: ident [ $( $ch: expr => $token_ty: ident ),* ] ) => {
                match self.peek_char {
                    $(
                        Some($ch) => {
                            self.read_char(); // Consume current_char
                            self.read_char(); // Consume peek_char
                            Token::$token_ty
                        }
                    )*
                    _ => token!($token_ty_default)
                }
            };
            ( $token_ty: ident ) => {{
                self.read_char();
                Token::$token_ty
            }};
        }

        match self.current_char {
            Some(ch) => match ch {
                '\n' => token!(NewLine),
                '=' => token!(Assign ['=' => Equal, '>' => FatArrow]),
                '!' => token!(Bang ['=' => NotEqual]),
                '|' => token!(Pipe ['|' => Or]),
                '&' => token!(Ampersant ['&' => And]),
                '@' => token!(WhoKnowsWhatThisIs),
                '#' => token!(Hash),
                '?' => token!(QuestionMark),
                '+' => token!(Plus ['+' => Increase]),
                '-' => token!(Minus ['>' => ThinArrow, '-' => Decrease]),
                '/' => token!(Slash),
                '*' => token!(Asterisk),
                '%' => token!(InAHundred),
                '>' => token!(GreaterThan ['=' => GreatEqual]),
                '<' => token!(LessThan ['=' => LessEqual]),
                ',' => token!(Comma),
                '.' => token!(Notation ['.' => TwoDots]),
                ';' => token!(SemiColon),
                ':' => token!(Colon [':' => Namespace]),
                '(' => token!(LParen),
                ')' => token!(RParen),
                '[' => token!(LBracket),
                ']' => token!(RBracket),
                '{' => token!(LBrace),
                '}' => token!(RBrace),
                c => {
                    self.read_char();
                    Token::Illegal(Literal::from(c.to_string()))
                }
            },
            None => Token::EOF,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            Some(ch) if ch.is_alphabetic() || ch == '_' => self.ident_token(),
            Some(ch) if ch.is_ascii_digit() => self.number_token(),
            Some('/') if matches!(self.peek_char, Some('/') | Some('*')) => {
                self.skip_comments();
                self.next_token()
            }
            Some('\'') => self.char_token(),
            Some('"') => self.string_token(),
            Some(_) => self.parse_token(),
            None => Token::EOF,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::EOF => None,
            t => Some(t),
        }
    }
}
