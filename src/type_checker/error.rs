use std::{error::Error, fmt::Display};

use crate::{
    Literal,
    expressions::{InfixOperator, PrefixOperator},
    types::Type,
};

#[derive(Debug)]
pub enum CheckerError {
    Undefined(Literal),
    Unsatisfied(Type, Type),
    InfixTypeMismatched(Type, InfixOperator, Type),
    PrefixTypeMismatched(PrefixOperator, Type),
}

pub type CheckerResult<T> = Result<T, CheckerError>;

#[derive(Debug)]
pub struct ProgramCheckError {
    all: Vec<CheckerError>,
}

impl ProgramCheckError {
    pub fn new<U: IntoIterator>(errors: U) -> Self
    where
        Vec<CheckerError>: FromIterator<U::Item>,
    {
        Self {
            all: errors.into_iter().collect(),
        }
    }
}

impl Error for CheckerError {}
impl Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CheckerError as E;

        f.write_str(&match self {
            E::Undefined(ident) => format!("`{ident}` is not found in the current scope"),
            E::Unsatisfied(t1, t2) => format!("Type `{t1}` does not satisfy the type `{t2}`"),
            E::InfixTypeMismatched(left_ty,operator, right_ty) => format!("Left type `{left_ty}` of `{operator}` infix operator doesn't match with type `{right_ty}`"),
            E::PrefixTypeMismatched(operator,  right_ty) => format!("Operator `{operator}` does not expects a value type of `{right_ty}`")
        })
    }
}

impl Error for ProgramCheckError {}
impl Display for ProgramCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.all {
            f.write_str(&err.to_string())?;
        }
        Ok(())
    }
}
