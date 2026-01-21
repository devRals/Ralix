use crate::{Expression, Literal, Parser, ParserError, ParserResult};

impl Parser<'_> {
    pub fn parse_string_literal(&mut self, int_lit: Literal) -> ParserResult<Expression> {
        Ok(Expression::String(int_lit.clone()))
    }

    pub fn parse_char_literal(&mut self, char_lit: Literal) -> ParserResult<Expression> {
        let value: char = match char_lit.parse().ok() {
            Some(v) => v,
            None => return Err(ParserError::FloatParse(char_lit.clone())),
        };

        Ok(Expression::Char(value))
    }
}
