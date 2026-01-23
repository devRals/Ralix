use std::fmt::Display;

use crate::{
    Literal,
    expressions::{ElseConsequence, Identifier, IfConsequence},
    types::Type,
};

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

#[derive(Debug, Clone)]
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

    Scope {
        statements: Vec<Statement>,
    },

    IfElse {
        consequences: Vec<IfConsequence>,
        else_consequence: Option<ElseConsequence>,
    },
}

pub struct Program {
    pub statements: Vec<Statement>,
}
