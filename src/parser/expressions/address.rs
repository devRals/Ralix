use crate::{Expression, Parser, ParserResult};

impl Parser<'_> {
    pub fn parse_address_expression(&mut self) -> ParserResult<Expression> {
        self.expect_ident()?;
        let ident = self.parse_identifier()?;

        Ok(Expression::AddrOf(ident))
    }
}
