use crate::{
    Expression, Parser, ParserError, ParserResult, Token,
    expressions::{InfixOperator, PrefixOperator},
    parser::expressions::Precedence,
};

impl Parser<'_> {
    pub fn parse_infix_expression(&mut self, left: Expression) -> ParserResult<Expression> {
        let operator = match &self.current_token {
            Token::Plus => InfixOperator::Add,
            Token::Minus => InfixOperator::Subtract,
            Token::Asterisk => InfixOperator::Multiply,
            Token::Slash => InfixOperator::Divide,
            Token::InAHundred => InfixOperator::Remainder,
            Token::Or => InfixOperator::Or,
            Token::And => InfixOperator::And,
            Token::Equal => InfixOperator::Equals,
            Token::NotEqual => InfixOperator::NotEquals,
            Token::GreaterThan => InfixOperator::Greater,
            Token::GreatEqual => InfixOperator::GreatEq,
            Token::LessThan => InfixOperator::Less,
            Token::LessEqual => InfixOperator::LessEq,
            t => return Err(ParserError::UnknownInfixOp(t.literal())),
        };

        let precedence = Precedence::of(&self.current_token);
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(Expression::Infix {
            left: left.into(),
            operator,
            right: right.into(),
        })
    }

    pub fn parse_prefix_expression(&mut self) -> ParserResult<Expression> {
        let operator = match &self.current_token {
            Token::Bang | Token::Not => PrefixOperator::Not,
            Token::Minus => PrefixOperator::Neg,
            Token::Asterisk => PrefixOperator::Deref,
            t => return Err(ParserError::UnknownInfixOp(t.literal())),
        };

        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;

        Ok(Expression::Prefix {
            operator,
            right: Box::new(right),
        })
    }

    pub fn parse_lparen_items(&mut self) -> ParserResult<Expression> {
        self.consume_current_token(Token::LParen);
        if self.is_current_token(Token::RParen) {
            return Ok(Expression::Null);
        }

        let expr = self.parse_expression(Precedence::Lowest)?;

        self.next_token();
        if self.is_current_token(Token::RParen) {
            Ok(expr)
        } else {
            todo!("Tuple expressions not implemented... YET!")
        }
    }
}
