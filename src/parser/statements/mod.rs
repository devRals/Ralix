use crate::{Parser, ParserResult, Statement, Token};

mod assignment;
mod binding;
mod expression;
mod function;

impl Parser<'_> {
    pub fn parse_statement(&mut self) -> ParserResult<Statement> {
        match &self.current_token {
            // Token::Ident(ident) => {
            //     if let Some(ty) = self.symbol_table.resolve(ident)
            //         ty == Type::Class {..} || ty == Type::Interface {..}
            //     {
            //
            //     }
            // }
            Token::TyInt
            | Token::TyFloat
            | Token::TyString
            | Token::TyChar
            | Token::Bool
            | Token::Type => self.parse_binding_statement(),
            Token::Function => self.parse_function_statement(),
            _ => self.parse_expression_statement(),
        }
    }
}
