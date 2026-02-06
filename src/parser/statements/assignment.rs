use crate::{
    Expression, Parser, ParserError, ParserResult, Statement, Token,
    parser::expressions::Precedence,
};

impl Parser<'_> {
    pub fn parse_assignment_statement(&mut self, expr: Expression) -> ParserResult<Statement> {
        match &expr {
            Expression::Identifier(_) => {}
            Expression::Index { .. } => {}
            _ => return Err(ParserError::CannotAssignTo(expr)),
        };

        self.skip_peek_token(Token::Assign);

        let value = self.parse_expression(Precedence::Lowest)?;

        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Assign { left: expr, value })
    }
}
