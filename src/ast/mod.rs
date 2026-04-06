use std::path::PathBuf;

use serde::Serialize;

use crate::{
    Literal,
    expressions::{
        ElseConsequence, FunctionParameter, HashMapItem, Identifier, IfConsequence, ImportedItem,
    },
    statements::Binding,
    types::{Type, TypeVarId},
};

pub mod expressions;
mod impls;
pub mod statements;
pub mod types;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Return(Option<Expression>),

    Binding(Binding),

    Assign {
        left: Expression,
        value: Expression,
    },

    Alias {
        ident: Identifier,
        ty: Type,
    },

    Get {
        module_name: Identifier,
        path_names: Vec<Identifier>,
        file_module_path: PathBuf,
        imported_items: Vec<ImportedItem>,
    },

    Out(Box<Statement>),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Expression {
    Identifier(expressions::Identifier),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(Literal),
    Char(char),
    Null,
    TypeOf(Box<Expression>),
    Type(Type),
    Try(Box<Expression>),

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
        generics: Vec<TypeVarId>,
        parameters: Vec<FunctionParameter>,
        return_type: Type,
        body: Box<Expression>,
    },

    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },

    Array {
        items: Vec<Expression>,
    },

    HashMap {
        items: Vec<HashMapItem>,
    },

    Index {
        left: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Serialize, Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
