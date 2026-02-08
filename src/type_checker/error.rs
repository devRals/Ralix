use std::{error::Error, fmt::Display};

use crate::{
    Literal,
    expressions::{Identifier, InfixOperator, PrefixOperator},
    types::Type,
};

#[derive(Debug, PartialEq)]
pub enum CheckerError {
    Undefined(Literal),
    Unsatisfied(Type, Type),
    InfixTypeMismatched(Type, InfixOperator, Type),
    PrefixTypeMismatched(PrefixOperator, Type),
    CannotDereference(Type),
    IfBranchesUnsatisfied(Type, Type),
    CannotBeCalled(Type),
    CannotbeIndexedBy(Type, Type),
    CannotBeHashed(Type),
    MismatchedArgumentCount(usize, usize),
    TypeofHadNullableExpr,
    AlreadyDefinedConstant(Identifier),
    AlreadyDefined(Identifier),
    IsAConstant(Identifier),
    UnavailableForCast(Type, Type),
    ArrayHasMultipleDifferentType(Type, Type),
    HashMaphHasMultipleDifferentKeyTypes(Type, Type),
    HashMaphHasMultipleDifferentValueTypes(Type, Type),
    IsNotNullable(Type),
    CannotUseTry(Type),
    InfiniteType,
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
            E::PrefixTypeMismatched(operator,  right_ty) => format!("Operator `{operator}` does not expects a value type of `{right_ty}`"),
            E::CannotDereference(t) => format!("Type `{t}` cannot be dereferenced"),
            E::IfBranchesUnsatisfied(expected, got) => format!("Type `{expected}` because of the previous if branches but the type `{got}` doesn't satisfy it"),
            E::CannotBeCalled(t) => format!("Type `{t}` is not callable"),
            E::MismatchedArgumentCount(expected, got) => format!("A function expected `{expected}` arguments but got `{got}`"),
            E::TypeofHadNullableExpr => "`typeof` expression cannot accept \"nullable\" expressions".to_string(),
            E::AlreadyDefinedConstant(ident) => format!("`{ident}` is already defined in the current scope as a \"constant\" and cannot be overwritten. Try using an another identifier name"),
            E::AlreadyDefined(ident) => format!("`{ident}` is already defined in the current scope. Try using an another identifier name"),
            E::IsAConstant(ident) => format!("Cannot assign a value to `{ident}` because it's a \"constant\""),
            E::UnavailableForCast(t1, t2) => format!("Cannot cast a value typeof `{t2}` to a type `{t1}`"),
            E::ArrayHasMultipleDifferentType(t1, t2) => format!("Array has multiple types of values, `{t1}` and `{t2}`"),
            E::CannotbeIndexedBy(t1, t2) => format!("Type `{t1}` cannot be indexed by `{t2}`"),
            E::CannotBeHashed(t) => format!("Type `{t}` cannot be used as key in a hashmap"),
            E::HashMaphHasMultipleDifferentKeyTypes(t1, t2) => format!("Hashmap has multipe types of keys, `{t1}` and `{t2}`"),
            E::HashMaphHasMultipleDifferentValueTypes(t1, t2) => format!("Hashmap has multipe types of values, `{t1}` and `{t2}`"),
            E::IsNotNullable(ty) => format!("Try expression expects a nullable type but got `{ty}`"),
            E::CannotUseTry(ty) => format!("Try expression cannot be used in a function that returns a value type of `{ty}`"),
            E::InfiniteType => "Recursive type variables are not allowed".to_string(),

        })
    }
}

impl Error for ProgramCheckError {}
impl Display for ProgramCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in &self.all {
            writeln!(f, "{err}")?;
        }
        Ok(())
    }
}
