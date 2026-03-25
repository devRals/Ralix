use crate::{Expression, Literal, Parser, ParserError, ParserResult};

impl Parser<'_> {
    pub fn parse_integer_literal(&mut self, int_lit: Literal) -> ParserResult<Expression> {
        // I'm sure there's a better way but ya know... lazy me :3
        let value = if int_lit.starts_with("0x") {
            i64::from_str_radix(int_lit.strip_prefix("0x").unwrap(), 16).ok()
        } else if int_lit.starts_with("0b") {
            i64::from_str_radix(int_lit.strip_prefix("0b").unwrap(), 2).ok()
        } else if int_lit.starts_with("0o") {
            i64::from_str_radix(int_lit.strip_prefix("0o").unwrap(), 8).ok()
        } else {
            int_lit.parse().ok()
        };

        Ok(Expression::Integer(match value {
            Some(v) => v,
            None => return Err(ParserError::IntegerParse(int_lit.clone())),
        }))
    }

    pub fn parse_float_literal(&mut self, float_lit: Literal) -> ParserResult<Expression> {
        let value: f64 = match float_lit.parse().ok() {
            Some(v) => v,
            None => return Err(ParserError::FloatParse(float_lit.clone())),
        };

        Ok(Expression::Float(value))
    }
}
