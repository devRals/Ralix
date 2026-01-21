use crate::{Literal, Parser, ParserError, ParserResult, Token, types::Type};

mod type_parsers;

impl Parser<'_> {
    pub fn parse_type_definition(&mut self) -> ParserResult<Type> {
        let initial_type = match &self.current_token {
            Token::TyInt => Type::Int,
            Token::Bool => Type::Bool,
            Token::TyChar => Type::Char,
            Token::TyFloat => Type::Float,
            Token::TyString => Type::String,
            Token::Ident(literal) => match self.symbol_table.resolve(literal) {
                Some(t) => t.clone(),
                None => {
                    return Err(ParserError::Undefined(Literal::from(
                        self.current_token.literal(),
                    )));
                }
            },
            t => {
                return Err(ParserError::TypeMistake(Literal::from(t.literal())));
            }
        };

        self.parse_primary_type(initial_type)
    }

    fn parse_primary_type(&mut self, initial_type: Type) -> ParserResult<Type> {
        let ty = match &self.peek_token {
            Token::Asterisk => {
                self.next_token();
                Type::Addr(Box::new(initial_type))
            }
            _ => return Ok(initial_type),
        };

        self.parse_primary_type(ty)
    }
}
