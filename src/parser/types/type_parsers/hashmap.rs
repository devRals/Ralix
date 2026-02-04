use crate::{Parser, ParserError, ParserResult, Token, types::Type};

impl Parser<'_> {
    pub fn parse_hashmap_type_definition(&mut self) -> ParserResult<Type> {
        self.expect_token(Token::LBracket)?;

        let key = self.parse_type_definition()?;
        self.expect_token(Token::Comma)?;
        let value = self.parse_type_definition()?;

        self.next_token();
        if !self.is_current_token(Token::RBracket) {
            return Err(ParserError::SyntaxError {
                expected: Token::RBracket,
                got: self.current_token.clone(),
            });
        }

        Ok(Type::HashMap {
            key: key.into(),
            value: value.into(),
        })
    }
}
