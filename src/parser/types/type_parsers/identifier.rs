use crate::{Literal, Parser, ParserError, ParserResult, types::Type};

impl Parser<'_> {
    pub fn parse_identifier_type_definition(&mut self, literal: Literal) -> ParserResult<Type> {
        match self.symbol_table.resolve(&literal) {
            Some(o) => Ok(o.ty.clone()),
            None => ParserResult::Err(ParserError::Undefined(literal)),
        }
    }
}
