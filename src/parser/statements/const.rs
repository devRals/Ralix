use crate::{ParserDiagnostic, ParserResult, Statement, Token, statements::Binding};

use super::Parser;

impl Parser<'_> {
    pub fn parse_constant_statements(&mut self) -> ParserResult<Statement> {
        self.consume_current_token(Token::Const);
        let stmt = self.parse_statement()?;

        match stmt {
            Statement::Binding(Binding {
                ident,
                type_annotation,
                value,
                ..
            }) => Ok(Statement::Binding(Binding {
                ident,
                type_annotation,
                value,
                is_constant: true,
            })),
            _ => Err(ParserDiagnostic::UnacceptableConst),
        }
    }
}
