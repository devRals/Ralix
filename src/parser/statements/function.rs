use crate::{
    Expression, Parser, ParserResult, Statement, Token, parser::expressions::Precedence,
    types::Type,
};

impl Parser<'_> {
    /// Not really a function statement it's just a binding statement that
    /// the binding value and type is a function
    pub fn parse_function_statement(&mut self) -> ParserResult<Statement> {
        self.expect_ident()?;

        let ident = self.parse_identifier()?;

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
