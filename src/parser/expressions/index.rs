use crate::{
    Expression, Parser, ParserError, ParserResult, Token, parser::expressions::Precedence,
};

impl Parser<'_> {
    pub fn parse_index_expression(&mut self, left: Expression) -> ParserResult<Expression> {
        self.consume_current_token(Token::LBracket);

        let index = self.parse_expression(Precedence::Lowest)?;

        self.next_token();
        if !self.is_current_token(Token::RBracket) {
            return Err(ParserError::SyntaxError {
                expected: Token::RBracket,
                got: self.current_token.clone(),
            });
        }

        Ok(Expression::Index {
            left: Box::new(left),
            index: Box::new(index),
        })
    }
}
