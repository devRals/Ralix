use crate::{Parser, ParserError, ParserResult, Token, types::Type};

impl Parser<'_> {
    pub fn parse_type_as_value_type_definition(&mut self) -> ParserResult<Type> {
        self.expect_token(Token::LBracket)?;

        let ty = self.parse_type_definition()?;

        self.next_token();
        if !self.is_current_token(Token::RBracket) {
            return Err(ParserError::SyntaxError {
                expected: Token::RBracket,
                got: self.current_token.clone(),
            });
        }
        Ok(Type::AsValue(ty.into()))
    }
}
