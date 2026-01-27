use crate::{
    Expression, Parser, ParserError, ParserResult, Statement, Token,
    parser::expressions::Precedence, types::Type,
};

impl Parser<'_> {
    /// Not really a function statement it's just a binding statement that
    /// the binding value and type is a function
    pub fn parse_function_statement(&mut self) -> ParserResult<Statement> {
        if self.is_peek_token(Token::LParen) {
            return Ok(Statement::Expression(self.parse_function_expression()?));
        }
        self.expect_ident()?;

        let ident = self.parse_identifier()?;

        self.expect_token(Token::LParen)?;
        let parameters = self.parse_function_parameters()?;
        self.consume_current_token(Token::RParen);

        let mut return_type = Type::Void;
        if !self.is_current_token(Token::Colon) {
            if !self.is_current_token(Token::ThinArrow) {
                return Err(ParserError::SyntaxError {
                    expected: Token::ThinArrow,
                    got: self.current_token.clone(),
                });
            }
            self.consume_current_token(Token::ThinArrow);
            return_type = self.parse_type_definition()?;
            self.expect_token(Token::Colon)?;
        } else {
            self.next_token();
        }

        let body = Box::new(self.parse_expression(Precedence::Lowest)?);

        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Binding {
            ident,
            type_annotation: None,
            value: Expression::Function {
                parameters,
                return_type,
                body,
            },
        })
    }
}
