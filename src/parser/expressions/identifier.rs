use crate::{Parser, ParserError, ParserResult, Token, expressions::Identifier};

impl Parser<'_> {
    pub fn parse_identifier(&mut self) -> ParserResult<Identifier> {
        let Token::Ident(name) = &self.current_token else {
            return Err(ParserError::IsNotIdentifier(self.current_token.clone()));
        };

        Ok(name.clone())
    }
}
