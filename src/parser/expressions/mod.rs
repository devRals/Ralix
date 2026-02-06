use crate::{Expression, Parser, ParserError, ParserResult, Token, types::Type};

mod address;
mod array;
mod copy;
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
    /// Default expression parsing precedence
    Lowest,
    // `true || false`
    BooleanLogic,
    /// `true != false, 1 == 1`
    Equals,
    /// `10 > 9; 99 < 100;`
    LessGreater,
    /// `10 - 8; 30 - 25;`
    Sum,
    /// `2 * 5; 10 / 2;`
    Product,
    /// `!true; -10; *ptr;`
    Prefix,
    /// `func(param1, param2);`
    Call,
    /// Index `arr[2] || hash_map["key"]`;
    /// Namespace `Enum::Item`;
    /// Notation `Class.attribute`
    Access,
}

impl Precedence {
    fn of(token: &Token) -> Precedence {
        use Token as T;
        match token {
            T::Equal | T::NotEqual => Precedence::Equals,
            T::LessThan | T::GreaterThan | T::LessEqual | T::GreatEqual => Precedence::LessGreater,
            T::Or | T::And => Precedence::BooleanLogic,
            T::Plus | T::Minus => Precedence::Sum,
            T::Asterisk | T::Slash => Precedence::Product,
            T::InAHundred => Precedence::Product,
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
            Token::Minus | Token::Bang | Token::Not | Token::Asterisk => {
                self.parse_prefix_expression()?
            }
            Token::LBrace => self.parse_scope_expression()?,
            Token::LParen => self.parse_lparen_items()?,
            Token::Copy => self.parse_copy_expression()?,
            Token::TypeOf => self.parse_typeof_expression()?,
            Token::Ampersant => self.parse_address_expression()?,
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
                | Token::InAHundred
                | Token::Plus
                | Token::Minus
                | Token::Equal
                | Token::NotEqual
                | Token::Or
                | Token::And
                | Token::LessThan
                | Token::LessEqual
                | Token::GreaterThan
                | Token::GreatEqual => self.parse_infix_expression(initial_expression)?,
                Token::LParen => self.parse_call_expression(initial_expression)?,
                Token::LBracket => self.parse_index_expression(initial_expression)?,
                Token::QuestionMark => self.parse_try_expression(initial_expression)?,

                _ => initial_expression,
            }
        }
        Ok(initial_expression)
    }
}
