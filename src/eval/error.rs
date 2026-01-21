use std::{error::Error, fmt::Display};

use crate::{
    Literal,
    expressions::{InfixOperator, PrefixOperator},
    types::Type,
};

#[derive(Debug, Clone)]
pub enum EvaluationError {
    Undefined(Literal),
    UnsupportedCopyType(Type),

    UnsupportedInfixOperation(Type, InfixOperator, Type),
    UnsupportedPrefixOperation(PrefixOperator, Type),

    CannotBeDereferenced(Type),
}

#[derive(Debug)]
pub enum EvalResult<T> {
    Value(T),
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
            E::UnsupportedPrefixOperation(op, right_ty) => {
                format!("Operator `{op}` is not supported for type `{right_ty}`")
            }
            E::CannotBeDereferenced(t) => format!("Type `{t}` cannot be dereferenced"),
        })
    }
}
