use std::rc::Rc;

use serde::Serialize;

use crate::{Expression, types::Type};

pub type Identifier = Rc<str>;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum InfixOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equals,
    NotEquals,
    Or,
    And,
    Greater,
    GreatEq,
    Less,
    LessEq,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum PrefixOperator {
    Not,
    Neg,
    Deref,
}

/// (Condition, Consequence)
pub type IfConsequence = (Expression, Expression);
pub type ElseConsequence = Box<Expression>;

pub type FunctionParameter = (Type, Identifier);
