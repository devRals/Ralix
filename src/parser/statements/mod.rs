use crate::{Parser, ParserResult, Statement, Token};

mod alias;
mod assignment;
mod binding;
mod r#const;
mod expression;
mod function;
mod r#return;

impl Parser<'_> {
    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        match &self.current_token {
            Token::Type if matches!(self.peek_token, Token::Ident(_)) => {
                self.parse_type_alias_statement()
            }
            Token::Const => self.parse_constant_statements(),
            Token::Function => self.parse_function_statement(),
            Token::Return => self.parse_return_statement(),
            Token::TyInt
            | Token::TyFloat
            | Token::TyString
            | Token::TyChar
            | Token::TyBool
            | Token::TyArr
            | Token::TyMap
            | Token::Type
            | Token::Let
            | Token::Ident(_) => self.parse_binding_statement(),
            _ => self.parse_expression_statement(),
        }
    }
}
