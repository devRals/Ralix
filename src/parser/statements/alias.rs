use crate::{Parser, ParserResult, Statement, Token, types::Type};

impl Parser<'_> {
    pub fn parse_type_alias_statement(&mut self) -> ParserResult<Statement> {
        self.expect_ident()?;

        let ident = self.parse_identifier()?;

        self.expect_token(Token::Assign)?;

        let ty = self.parse_type_definition()?;

        self.consume_peek_token(Token::SemiColon);

        self.symbol_table
            .define(ident.clone(), Type::AsValue(ty.clone().into()), true);
        Ok(Statement::Alias { ident, ty })
    }
}
