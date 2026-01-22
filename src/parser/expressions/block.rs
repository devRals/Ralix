use crate::{Expression, Parser, ParserResult, Token};

impl Parser<'_> {
    pub fn parse_block_expression(&mut self) -> ParserResult<Expression> {
        let mut statements = Vec::new();
        self.consume_current_token(Token::LBrace);
        self.symbol_table.enter_scope();

        loop {
            if self.is_current_token(Token::RBrace) {
                self.symbol_table.leave_scope();
                break Ok(Expression::Scope { statements });
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }
    }
}
