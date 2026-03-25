use crate::{Expression, Parser, ParserError, ParserResult, Token, types::Type};

mod array;
mod function;
mod hashmap;
mod identifier;
mod if_else;
mod index;
mod infix_prefix;
mod number;
mod scope;
mod string;
mod r#try;
mod types;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest, // Default expression parsing precedence

    LogicalOr,  // ||
    LogicalAnd, // &&

    BitwiseOr,  // |
    BitwiseXOr, // ^
    BitwiseAnd, // &

    Equals,      // !=  ==
    LessGreater, // > >= < <=

    Shift, // <<, >>

    Sum,     // + -
    Product, // * / %

    Prefix, // ! - * ~

    Call, // `func(param1, param2);`
    Access, // Index `arr[2] || hash_map["key"]`;
          // Namespace `Enum::Item`;
          // Notation `Class.attribute`
}

impl Precedence {
    fn of(token: &Token) -> Precedence {
        use Token as T;
        match token {
            T::Or => Precedence::LogicalOr,
            T::And => Precedence::LogicalAnd,

            T::Pipe => Precedence::BitwiseOr,
            T::Caret => Precedence::BitwiseXOr,
            T::Ampersant => Precedence::BitwiseAnd,

            T::Equal | T::NotEqual => Precedence::Equals,
            T::LessThan | T::GreaterThan | T::LessEqual | T::GreatEqual => Precedence::LessGreater,

            T::ShiftLeft | T::ShiftRight => Precedence::Shift,

            T::Plus | T::Minus => Precedence::Sum,
            T::Asterisk | T::Slash | T::Percent => Precedence::Product,

            /* Prefix is seperated in it's own parser function */
            T::LParen => Precedence::Call,
            T::LBracket | T::Namespace | T::Notation | T::QuestionMark => Precedence::Access,

            _ => Precedence::Lowest,
        }
    }
}

impl Parser<'_> {
    pub fn parse_expression(&mut self, precedence: Precedence) -> ParserResult<Expression> {
        let initial_expression = match self.current_token.clone() {
            Token::String(str_lit) => self.parse_string_literal(str_lit)?,
            Token::Char(ch_lit) => self.parse_char_literal(ch_lit)?,
            Token::Int(int_lit) => self.parse_integer_literal(int_lit)?,
            Token::Float(float_lit) => self.parse_float_literal(float_lit)?,
            Token::True => Expression::Boolean(true),
            Token::False => Expression::Boolean(false),
            Token::Null => Expression::Null,
            Token::Ident(ident) => Expression::Identifier(ident),
            Token::TyInt | Token::TyChar | Token::TyBool | Token::TyFloat | Token::TyString => {
                Expression::Type(Type::from_token(&self.current_token).unwrap())
            }
            Token::Minus
            | Token::Bang
            | Token::Not
            | Token::Asterisk
            | Token::Ampersant
            | Token::Tilde => self.parse_prefix_expression()?,
            Token::LBrace => self.parse_scope_expression()?,
            Token::LParen => self.parse_lparen_items()?,
            Token::TypeOf => self.parse_typeof_expression()?,
            Token::If => self.parse_if_expression()?,
            Token::Function => self.parse_function_expression()?,
            Token::LBracket => self.parse_array_literal()?,
            Token::Hash => self.parse_hashmap_literal()?,
            t => return Err(ParserError::ExpressionMistake(t.clone())),
        };

        self.parse_primary_expressions(initial_expression, precedence)
    }

    pub fn parse_primary_expressions(
        &mut self,
        mut initial_expression: Expression,
        precedence: Precedence,
    ) -> ParserResult<Expression> {
        while precedence < Precedence::of(&self.peek_token) {
            self.next_token();
            initial_expression = match self.current_token {
                Token::Asterisk
                | Token::Slash
                | Token::Percent
                | Token::Plus
                | Token::Minus
                | Token::Equal
                | Token::NotEqual
                | Token::Or
                | Token::And
                | Token::LessThan
                | Token::LessEqual
                | Token::GreaterThan
                | Token::GreatEqual
                | Token::Ampersant
                | Token::Pipe
                | Token::Caret
                | Token::ShiftRight
                | Token::ShiftLeft => self.parse_infix_expression(initial_expression)?,
                Token::LParen => self.parse_call_expression(initial_expression)?,
                Token::LBracket => self.parse_index_expression(initial_expression)?,
                Token::QuestionMark => self.parse_try_expression(initial_expression)?,

                _ => initial_expression,
            }
        }
        Ok(initial_expression)
    }
}
