use crate::{Expression, Literal, Parser, ParserError, ParserResult};

impl Parser<'_> {
    pub fn parse_integer_literal(&mut self, int_lit: Literal) -> ParserResult<Expression> {
        let value: i64 = match int_lit.parse().ok() {
            Some(v) => v,
            None => return Err(ParserError::IntegerParse(int_lit.clone())),
        };

        Ok(Expression::Integer(value))
    }

    pub fn parse_float_literal(&mut self, float_lit: Literal) -> ParserResult<Expression> {
        let value: f64 = match float_lit.parse().ok() {
            Some(v) => v,
            None => return Err(ParserError::FloatParse(float_lit.clone())),
        };

        Ok(Expression::Float(value))
    }
}
