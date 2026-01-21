use crate::{Expression, Parser, ParserResult, Token, parser::expressions::Precedence};

impl Parser<'_> {
    pub fn parse_typeof_expression(&mut self) -> ParserResult<Expression> {
        self.consume_current_token(Token::TypeOf);

        let val = self.parse_expression(Precedence::Lowest)?;

        Ok(Expression::TypeOf(Box::new(val)))
    }
}
