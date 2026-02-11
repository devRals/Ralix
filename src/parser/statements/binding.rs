use crate::{
    Expression, Parser, ParserError, ParserResult, Statement, Token,
    parser::expressions::Precedence, types::Type,
};

impl Parser<'_> {
    pub fn parse_binding_statement(&mut self) -> ParserResult<Statement> {
        if self.is_current_token(Token::Let) {
            return self.parse_let_binding();
        }

        // Please don't hate me
        let type_annotation = match &self.current_token {
            Token::Ident(ident) => match self.symbol_table.resolve_ref(ident) {
                Some(v) => match &v.ty {
                    Type::AsValue(ty) => self.parse_primary_type(*ty.clone())?,
                    _ => {
                        let expr = Expression::Identifier(ident.clone());
                        let primary_expr =
                            self.parse_primary_expressions(expr, Precedence::Lowest)?;

                        return if self.is_peek_token(Token::Assign) {
                            self.parse_assignment_statement(primary_expr)
                        } else {
                            Ok(Statement::Expression(primary_expr))
                        };
                    }
                },
                None => {
                    let expr = Expression::Identifier(ident.clone());
                    let primary_expr = self.parse_primary_expressions(expr, Precedence::Lowest)?;

                    return if self.is_peek_token(Token::Assign) {
                        self.parse_assignment_statement(primary_expr)
                    } else {
                        Ok(Statement::Expression(primary_expr))
                    };
                }
            },
            _ => self.parse_type_definition()?,
        };

        // It's not a binding
        if !matches!(&self.peek_token, Token::Ident(_)) {
            let expr = Expression::Type(type_annotation);
            let primary_expr = self.parse_primary_expressions(expr, Precedence::Lowest)?;
            return Ok(Statement::Expression(primary_expr));
        }

        self.expect_ident()?;
        let ident = self.parse_identifier()?;
        self.expect_token(Token::Assign)?;

        let value = self.parse_expression(Precedence::Lowest)?;
        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Binding {
            ident,
            type_annotation: Some(type_annotation),
            value,
            is_constant: false,
        })
    }

    pub fn parse_let_binding(&mut self) -> ParserResult<Statement> {
        self.expect_ident()?;
        let ident = self.parse_identifier()?;

        let mut type_annotation = None;
        if self.is_peek_token(Token::Colon) {
            self.skip_peek_token(Token::Colon);
            type_annotation = self.parse_type_definition()?.into();
        }

        self.expect_token(Token::Assign)?;

        let value = self.parse_expression(Precedence::Lowest)?;

        self.consume_peek_token(Token::SemiColon);

        Ok(Statement::Binding {
            ident,
            type_annotation,
            value,
            is_constant: false,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{Expression, Lexer, Literal, Parser, Statement, SymbolTable, types::Type};

    #[test]
    fn test_parse_binding() {
        let tests = [(
            "int a = 3;",
            Statement::Binding {
                is_constant: false,
                type_annotation: Some(Type::Int),
                ident: Literal::from("a"),
                value: Expression::Integer(3),
            },
        )];

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
