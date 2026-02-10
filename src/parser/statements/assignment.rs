use crate::{
    Expression, Parser, ParserError, ParserResult, Statement, Token, expressions::PrefixOperator,
    parser::expressions::Precedence,
};

impl Parser<'_> {
    pub fn parse_assignment_statement(&mut self, expr: Expression) -> ParserResult<Statement> {
        match &expr {
            Expression::Identifier(_) => {}
            Expression::Index { .. } => {}
            Expression::Prefix {
                operator: PrefixOperator::Deref,
                ..
            } => {}
            _ => return Err(ParserError::CannotAssignTo(expr)),
        };

        self.skip_peek_token(Token::Assign);

        let value = self.parse_expression(Precedence::Lowest)?;

        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Assign { left: expr, value })
    }
}
