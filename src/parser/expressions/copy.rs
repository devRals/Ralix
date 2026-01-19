use crate::{Expression, Parser, ParserResult, Token};

impl Parser<'_> {
    pub fn parse_copy_expression(&mut self) -> ParserResult<Expression> {
        self.consume_current(Token::Copy);

        let ident = self.parse_identifier()?;

        Ok(Expression::Copy(ident))
    }
}
