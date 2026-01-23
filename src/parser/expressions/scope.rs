use crate::{Expression, Parser, ParserResult, Token};

impl Parser<'_> {
    pub fn parse_scope_expression(&mut self) -> ParserResult<Expression> {
        self.consume_current_token(Token::LBrace);

        self.symbol_table.enter_scope();
        let mut statements = Vec::new();
        loop {
            if self.is_current_token(Token::RBrace) || self.is_current_token(Token::EOF) {
                self.symbol_table.leave_scope();
                break Ok(Expression::Scope { statements });
            }

            if let Ok(stmt) = self.parse_statement() {
                statements.push(stmt)
            }
            self.next_token();
        }
    }
}
