use crate::{
    Expression, Parser, ParserError, ParserResult, Token,
    expressions::{ElseConsequence, IfConsequence},
    parser::expressions::Precedence,
};

impl Parser<'_> {
    pub fn parse_if_expression(&mut self) -> ParserResult<Expression> {
        let mut consequences = Vec::new();
        let con = self.parse_if_consequence()?;
        consequences.push(con);

        let mut else_consequence = None;

        while self.is_peek_token(Token::Else) {
            self.skip_peek_token(Token::Else);

            if self.is_current_token(Token::If) {
                let con = self.parse_if_consequence()?;
                consequences.push(con);
            } else {
                if !self.is_current_token(Token::Colon) {
                    return Err(ParserError::SyntaxError {
                        expected: Token::Colon,
                        got: self.current_token.clone(),
                    });
                }
                self.next_token();

                let expr = self.parse_expression(Precedence::Lowest)?;
                else_consequence = Some(ElseConsequence::new(expr));
                break;
            }
        }

        Ok(Expression::IfElse {
            consequences,
            else_consequence,
        })
    }

    fn parse_if_consequence(&mut self) -> ParserResult<IfConsequence> {
        self.consume_current_token(Token::If);

        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_token(Token::Colon)?;
        let consequence = self.parse_expression(Precedence::Lowest)?;

        Ok(IfConsequence::from((condition, consequence)))
    }
}
