use crate::{ParserError, ParserResult, Statement, Token};

use super::Parser;

impl Parser<'_> {
    pub fn parse_constant_statements(&mut self) -> ParserResult<Statement> {
        self.consume_current_token(Token::Const);
        let stmt = self.parse_statement()?;

        match stmt {
            Statement::Binding {
                ident,
                type_annotation,
                value,
                ..
            } => Ok(Statement::Binding {
                ident,
                type_annotation,
                value,
                is_constant: true,
            }),
            _ => Err(ParserError::UnacceptableConst),
        }
    }
}
