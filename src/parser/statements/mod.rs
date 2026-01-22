use crate::{Parser, ParserResult, Statement, Token};

mod assignment;
mod binding;
mod expression;

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
            | Token::Let => self.parse_binding_statement(),
            _ => self.parse_expression_statement(),
        }
    }
}
