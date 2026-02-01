use crate::{
    Expression, ParserError, ParserResult, Token, expressions::FunctionParameter,
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
            Token::ThinArrow => {
                self.consume_current_token(Token::ThinArrow);
                let ty = self.parse_type_definition()?;
                self.expect_token(Token::Colon)?;
                ty
            }
            _ => {
                return Err(ParserError::SyntaxError {
                    expected: Token::ThinArrow,
                    got: self.current_token.clone(),
                });
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

            let is_constant = self.consume_current_token(Token::Const);
            let type_def = self.parse_type_definition()?;
            self.expect_ident()?;
            let name = self.parse_identifier()?;

            self.next_token();
            parameters.push(FunctionParameter {
                name,
                type_def,
                is_constant,
            });
        }
    }

    pub fn parse_call_expression(&mut self, function: Expression) -> ParserResult<Expression> {
        self.consume_current_token(Token::LParen);

        let mut arguments = Vec::new();

        loop {
            if self.is_current_token(Token::RParen) {
                break Ok(Expression::Call {
                    function: Box::new(function),
                    arguments,
                });
            }

            self.consume_current_token(Token::Comma);

            let arg_expr = self.parse_expression(Precedence::Lowest)?;
            arguments.push(arg_expr);
            self.next_token();
        }
    }
}
