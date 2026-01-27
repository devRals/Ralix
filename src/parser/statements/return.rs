use crate::{ParserResult, Statement, Token, parser::expressions::Precedence};

use super::Parser;

impl Parser<'_> {
    pub fn parse_return_statement(&mut self) -> ParserResult<Statement> {
        self.consume_current_token(Token::Return);

        let value = if self.is_current_token(Token::SemiColon) {
            None
        } else {
            Some(self.parse_expression(Precedence::Lowest)?)
        };

        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Return(value))
    }
}
