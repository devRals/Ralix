use crate::{Parser, ParserDiagnostic, ParserResult, Token, expressions::Identifier};

impl Parser<'_> {
    pub fn parse_identifier(&mut self) -> ParserResult<Identifier> {
        let Token::Ident(name) = &self.current_token else {
            return Err(ParserDiagnostic::IsNotIdentifier(
                self.current_token.clone(),
            ));
        };

        Ok(name.clone())
    }
}
