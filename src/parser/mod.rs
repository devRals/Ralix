use crate::{Lexer, Program, SymbolTable, Token};

mod error;
mod expressions;
mod statements;
mod types;

pub use error::*;

pub struct Parser<'src> {
    current_token: Token,
    peek_token: Token,

    lexer: Lexer<'src>,

    symbol_table: &'src mut SymbolTable,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src>, symbol_table: &'src mut SymbolTable) -> Self {
        let mut parser = Parser {
            lexer,

            peek_token: Token::default(),
            current_token: Token::default(),

            symbol_table,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, ProgramParseError> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !matches!(self.current_token, Token::EOF) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => errors.push(err),
            }

            self.next_token();
        }

        if !errors.is_empty() {
            Err(ProgramParseError::new(errors))
        } else {
            Ok(Program { statements })
        }
    }
}

impl Parser<'_> {
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_ident(&mut self) -> ParserResult<()> {
        self.next_token();
        if !matches!(self.current_token, Token::Ident(_)) {
            return Err(ParserError::IsNotIdentifier(self.current_token.clone()));
        }

        Ok(())
    }

    fn expect_token(&mut self, token: Token) -> ParserResult<()> {
        self.next_token();
        if token != self.current_token {
            return Err(ParserError::SyntaxError {
                expected: token,
                got: self.current_token.clone(),
            });
        }
        self.next_token();

        Ok(())
    }

    fn is_current_token(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn is_peek_token(&self, token: Token) -> bool {
        self.peek_token == token
    }

    fn consume_peek_token(&mut self, token: Token) {
        if self.is_peek_token(token) {
            self.next_token();
        }
    }

    fn consume_current_token(&mut self, token: Token) {
        if self.is_current_token(token) {
            self.next_token();
        }
    }

    fn skip_peek_token(&mut self, token: Token) {
        if self.is_peek_token(token) {
            self.next_token();
            self.next_token();
        }
    }
}
