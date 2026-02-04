use crate::{Parser, ParserResult, Statement, Token};

mod assignment;
mod binding;
mod r#const;
mod expression;
mod function;
mod r#return;

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
            | Token::TyBool
            | Token::TyArr
            | Token::TyMap
            | Token::Type
            | Token::Let => self.parse_binding_statement(),
            Token::Const => self.parse_constant_statements(),
            Token::Function => self.parse_function_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }
}
