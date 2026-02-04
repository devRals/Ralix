use crate::{
    Expression, Parser, ParserResult, Token, expressions::HashMapItem,
    parser::expressions::Precedence,
};

impl Parser<'_> {
    pub fn parse_hashmap_literal(&mut self) -> ParserResult<Expression> {
        self.expect_token(Token::LBrace)?;

        let mut items = Vec::new();

        loop {
            if self.is_current_token(Token::RBrace) {
                break Ok(Expression::HashMap { items });
            }

            self.consume_current_token(Token::Comma);

            let key = self.parse_expression(Precedence::Lowest)?;
            self.expect_token(Token::Colon)?;
            let value = self.parse_expression(Precedence::Lowest)?;

            items.push(HashMapItem { key, value });

            self.next_token();
        }
    }
}
