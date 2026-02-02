use serde::Serialize;

use crate::{
    Literal,
    expressions::{ElseConsequence, FunctionParameter, Identifier, IfConsequence},
    types::Type,
};

pub mod expressions;
mod impls;
pub mod types;

#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    Expression(Expression),
    Return(Option<Expression>),

    Binding {
        ident: Identifier,
        type_annotation: Option<Type>,
        value: Expression,
        is_constant: bool,
    },

    Assign {
        left: Expression,
        value: Expression,
    },
}

#[derive(Debug, Clone, Serialize)]
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

    Function {
        parameters: Vec<FunctionParameter>,
        return_type: Type,
        body: Box<Expression>,
    },

    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Serialize, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
