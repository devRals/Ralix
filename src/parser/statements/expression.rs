use crate::{Parser, ParserResult, Statement, parser::expressions::Precedence};

impl Parser<'_> {
    pub fn parse_expression_statement(&mut self) -> ParserResult<Statement> {
        Ok(Statement::Expression(
            self.parse_expression(Precedence::Lowest)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Expression, Lexer, Literal, Parser, Statement, SymbolTable};

    #[test]
    fn test_parse_expression_statement() {
        let tests = ["3", "3.14159", "true", "false", "'c'", r#" "hello world" "#];
        let expected = [
            Statement::Expression(Expression::Integer(3)),
            Statement::Expression(Expression::Float(
                #[allow(clippy::approx_constant)]
                3.14159,
            )),
            Statement::Expression(Expression::Boolean(true)),
            Statement::Expression(Expression::Boolean(false)),
            Statement::Expression(Expression::Char('c')),
            Statement::Expression(Expression::String(Literal::from("hello world"))),
        ];

        for (test, expected) in tests.into_iter().zip(expected) {
            let lexer = Lexer::new(test);
            let mut symbol_table = SymbolTable::default();
            let mut parser = Parser::new(lexer, &mut symbol_table);

            let result = parser
                .parse_expression_statement()
                .unwrap_or_else(|err| panic!("{err}"));

            assert_eq!(format!("{result}"), format!("{expected}"))
        }
    }
}
