use crate::{Literal, Parser, ParserError, ParserResult, Token, types::Type};

mod type_parsers;

impl Parser<'_> {
    pub fn parse_type_definition(&mut self) -> ParserResult<Type> {
        let initial_type = match &self.current_token {
            Token::TyInt => Type::Int,
            Token::TyBool => Type::Bool,
            Token::TyChar => Type::Char,
            Token::TyFloat => Type::Float,
            Token::TyString => Type::String,
            Token::TyArr => self.parse_array_type_definition()?,
            Token::TyMap => self.parse_hashmap_type_definition()?,
            Token::Type => self.parse_type_as_value_type_definition()?,
            Token::Function => self.parse_function_type_definition()?,
            Token::Ident(literal) => self.parse_identifier_type_definition(literal.clone())?,
            t => {
                return Err(ParserError::TypeMistake(Literal::from(t.literal())));
            }
        };

        self.parse_primary_type(initial_type)
    }

    fn parse_primary_type(&mut self, initial_type: Type) -> ParserResult<Type> {
        let ty = match &self.peek_token {
            Token::Asterisk => Type::Addr(Box::new(initial_type)),
            Token::QuestionMark => Type::Nullable(Box::new(initial_type)),
            _ => return Ok(initial_type),
        };

        self.next_token();

        self.parse_primary_type(ty)
    }
}
