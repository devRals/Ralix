use crate::{Parser, ParserResult, Token, types::TypeVarId};

impl Parser<'_> {
    pub fn parse_generics(&mut self) -> ParserResult<Vec<TypeVarId>> {
        self.expect_ident()?;

        let mut variable_ids = vec![TypeVarId {
            name: self.parse_identifier()?,
            id: self.symbol_table.crate_id(),
        }];

        loop {
            self.next_token();
            if self.is_current_token(Token::RBracket) {
                break Ok(variable_ids);
            }

            self.consume_current_token(Token::Comma);

            let name = self.parse_identifier()?;
            variable_ids.push(TypeVarId {
                name,
                id: self.symbol_table.crate_id(),
            })
        }
    }
}
