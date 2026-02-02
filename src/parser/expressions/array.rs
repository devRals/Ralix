use crate::{Expression, Parser, ParserResult, Token, parser::expressions::Precedence};

impl Parser<'_> {
    pub fn parse_array_literal(&mut self) -> ParserResult<Expression> {
        self.consume_current_token(Token::LBracket);
        let mut items = Vec::new();

        loop {
            if self.is_current_token(Token::RBracket) {
                break Ok(Expression::Array { items });
            }

            self.consume_current_token(Token::Comma);

            let expr = self.parse_expression(Precedence::Lowest)?;
            items.push(expr);
            self.next_token();
        }
    }
}
