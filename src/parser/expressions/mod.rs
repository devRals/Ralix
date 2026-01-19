use crate::{Expression, Parser, ParserError, ParserResult, Token};

mod copy;
mod identifier;
mod infix_prefix;
mod number;
mod string;

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
    /// `!true; -10;`
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
            T::LBracket | T::Namespace | T::Notation => Precedence::Access,
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
            Token::Minus | Token::Bang | Token::Not => self.parse_prefix_expression()?,
            Token::Copy => self.parse_copy_expression()?,
            t => return Err(ParserError::ExpressionMistake(t.clone())),
        };

        self.parse_infix_expressions(initial_expression, precedence)
    }

    fn parse_infix_expressions(
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
                | Token::And => self.parse_infix_expression(initial_expression)?,

                _ => initial_expression,
            }
        }
        Ok(initial_expression)
    }
}
