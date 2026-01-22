use std::fmt::Display;

use crate::{Literal, expressions::Identifier, types::Type};

pub mod expressions;
mod impls;
pub mod statements;
pub mod types;

pub trait Node: Display {
    fn downcast(self) -> NodeV;
}

pub enum NodeV {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    Binding(statements::Binding),
    Expression(Expression),

    Assign { left: Expression, value: Expression },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(expressions::Identifier),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(Literal),
    Char(char),
    Copy(Identifier),
    AddrOf(Identifier), // a basic Pointer
    Null,
    TypeOf(Box<Expression>),
    Type(Type),

    Infix {
        left: Box<Expression>,
        operator: expressions::InfixOperator,
        right: Box<Expression>,
    },
    Prefix {
        operator: expressions::PrefixOperator,
        right: Box<Expression>,
    },
}

pub struct Program {
    pub statements: Vec<Statement>,
}
