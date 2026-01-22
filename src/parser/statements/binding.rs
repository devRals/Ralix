use crate::{
    Expression, Parser, ParserResult, Statement, Token, parser::expressions::Precedence,
    statements::Binding,
};

impl Parser<'_> {
    pub fn parse_binding_statement(&mut self) -> ParserResult<Statement> {
        if matches!(self.current_token, Token::Let) {
            return self.parse_let_binding_statement();
        }

        let type_annotation = self.parse_type_definition()?;

        // It's not a binding
        if !matches!(&self.peek_token, Token::Ident(_)) {
            let expr = Expression::Type(type_annotation);
            let primary_expr = self.parse_primary_expressions(expr, Precedence::Lowest)?;
            return Ok(Statement::Expression(primary_expr));
        }

        self.expect_ident()?;
        let ident = self.parse_identifier()?;
        self.expect_token(Token::Assign)?;
        self.next_token(); // Consume Token::Assign

        let value = self.parse_expression(Precedence::Lowest)?;
        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Binding(Binding {
            ident,
            type_annotation: Some(type_annotation),
            value,
        }))
    }

    fn parse_let_binding_statement(&mut self) -> ParserResult<Statement> {
        self.expect_ident()?;

        let ident = self.parse_identifier()?;
        let mut type_annotation = None;

        if self.is_peek_token(Token::Colon) {
            self.skip_peek_token(Token::Colon);
            type_annotation = Some(self.parse_type_definition()?);
        }

        self.expect_token(Token::Assign)?;
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Binding(Binding {
            ident,
            value,
            type_annotation,
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Expression, Lexer, Literal, Parser, Statement, SymbolTable, statements::Binding,
        types::Type,
    };

    #[test]
    fn test_parse_binding() {
        let tests = [
            (
                "int a = 3;",
                Statement::Binding(Binding {
                    type_annotation: Some(Type::Int),
                    ident: Literal::from("a"),
                    value: Expression::Integer(3),
                }),
            ),
            (
                "let owo = \":3\" ",
                Statement::Binding(Binding {
                    ident: Literal::from("owo"),
                    type_annotation: None,
                    value: Expression::String(Literal::from(":3")),
                }),
            ),
        ];

        for (src, expected) in tests {
            let mut symbol_table = SymbolTable::default();
            let lexer = Lexer::new(src);
            let mut parser = Parser::new(lexer, &mut symbol_table);
            let stmt = parser
                .parse_binding_statement()
                .unwrap_or_else(|err| panic!("{err}"));

            assert_eq!(stmt.to_string(), expected.to_string());
        }
    }
}
