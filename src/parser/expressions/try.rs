use crate::{Expression, Parser, ParserResult};

impl Parser<'_> {
    pub fn parse_try_expression(&mut self, left: Expression) -> ParserResult<Expression> {
        Ok(Expression::Try(left.into()))
    }
}
