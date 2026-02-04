use std::{error::Error, fmt::Display};

use crate::{
    Expression, Literal, Object,
    expressions::{InfixOperator, PrefixOperator},
    types::Type,
};

#[derive(Debug, Clone)]
pub enum EvaluationError {
    Undefined(Literal),
    UnsupportedCopyType(Type),

    UnsupportedInfixOperation(Type, InfixOperator, Type),
    UnsupportedPrefixOperation(PrefixOperator, Type),
    UnsupportedIndexOperation(Type, Type),

    CannotBeDereferenced(Type),
    CannotAssign(Expression, Object),
    IsNotAFunction(Type),
}

#[derive(Debug)]
pub enum EvalResult<T> {
    Value(T),
    Return(Option<T>),
    Err(EvaluationError),
    NoValue,
}

impl Error for EvaluationError {}
impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EvaluationError as E;

        f.write_str(&match self {
            E::Undefined(lit) => format!("`{lit}` is undefined"),
            E::UnsupportedCopyType(ty) => format!("Cannot create a copy of type `{ty}`"),
            E::UnsupportedInfixOperation(left_ty, op, right_ty) => {
                format!("Operator `{op}` is not supported for types `{left_ty}` and `{right_ty}`")
            }
            E::UnsupportedIndexOperation(left, idx) => {
                format!("Index operation not supported for types `{left}` and `{idx}`")
            }
            E::UnsupportedPrefixOperation(op, right_ty) => {
                format!("Operator `{op}` is not supported for type `{right_ty}`")
            }
            E::CannotBeDereferenced(t) => format!("Type `{t}` cannot be dereferenced"),
            E::CannotAssign(left, value) => {
                format!("Cannot assign value {value} using {left} expression")
            }
            E::IsNotAFunction(t) => {
                format!("Object type `{t}` is not a function and cannot be called")
            }
        })
    }
}
