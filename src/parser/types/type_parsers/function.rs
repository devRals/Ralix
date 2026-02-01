use crate::{
    Parser, ParserResult, Token,
    types::{FunctionParameterType, Type},
};

impl Parser<'_> {
    pub fn parse_function_type_definition(&mut self) -> ParserResult<Type> {
        self.expect_token(Token::LParen)?;
        let mut parameters = Vec::new();
        let mut return_type = Type::Void;

        loop {
            if matches!(self.current_token, Token::RParen) {
                if self.is_peek_token(Token::ThinArrow) {
                    self.skip_peek_token(Token::ThinArrow);
                    return_type = self.parse_type_definition()?;
                }
                break Ok(Type::Function {
                    parameters,
                    return_type: return_type.into(),
                });
            }

            self.consume_current_token(Token::Comma);

            let mut is_constant = false;
            if self.is_current_token(Token::Const) {
                self.consume_current_token(Token::Const);
                is_constant = true;
            }

            let ty = self.parse_type_definition()?;
            parameters.push(FunctionParameterType { ty, is_constant });
            self.next_token();
        }
    }
}
