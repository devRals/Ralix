use std::fmt::Display;

use crate::{Literal, expressions::Identifier};

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
}

#[derive(Debug)]
pub enum Expression {
    Identifier(expressions::Identifier),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(Literal),
    Char(char),
    Copy(Identifier),
    Null,

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
