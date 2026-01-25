use crate::{
    Expression, ParserResult, Token, expressions::FunctionParameter,
    parser::expressions::Precedence, types::Type,
};

use super::Parser;

impl Parser<'_> {
    pub fn parse_function_expression(&mut self) -> ParserResult<Expression> {
        self.expect_token(Token::LParen)?;
        let parameters = self.parse_function_parameters()?;
        self.consume_current_token(Token::RParen);

        let return_type = match self.current_token {
            Token::Colon => {
                self.consume_current_token(Token::Colon);
                Type::Void
            }
            _ => {
                let ty = self.parse_type_definition()?;
                self.expect_token(Token::Colon)?;
                ty
            }
        };

        let body = Box::new(self.parse_expression(Precedence::Lowest)?);

        Ok(Expression::Function {
            parameters,
            return_type,
            body,
        })
    }

    pub fn parse_function_parameters(&mut self) -> ParserResult<Vec<FunctionParameter>> {
        let mut parameters = Vec::new();

        loop {
            if self.is_current_token(Token::RParen) {
                break Ok(parameters);
            }

            self.consume_current_token(Token::Comma);

            let type_def = self.parse_type_definition()?;
            self.expect_ident()?;
            let param_name = self.parse_identifier()?;

            self.next_token();
            parameters.push((type_def, param_name));
        }
    }

    pub fn parse_call_expression(
        &mut self,
        _initial_expression: Expression,
    ) -> ParserResult<Expression> {
        todo!()
    }
}
