use crate::expressions::{InfixOperator, PrefixOperator};
use std::{cmp, ops};

use crate::{EvalResult, EvaluationError, Object};

macro_rules! impl_infix_op {
    ($(
        [$op_trait: ty => $method: ident : {$(
            $objects: pat => $evaluation: expr
        ),*}, $op_err_ty: ident]
    ),*) => {
        $(
            impl $op_trait for Object {
                type Output = EvalResult<Object>;
                fn $method(self, other: Self) -> Self::Output {
                    use Object::*;
                    match (self, other) {
                        $(
                            $objects => $evaluation,
                        )*
                        (o1, o2) => return EvalResult::Err(EvaluationError::UnsupportedInfixOperation(
                            o1.r#type(), InfixOperator::$op_err_ty, o2.r#type()
                        ))
                    }.into()
                }
            }
        )*
    };
}

macro_rules! impl_prefix_op {
    ($(
        [$op_trait: ty => $method: ident : {$(
            $objects: pat => $evaluation: expr
        ),*}, $op_err_ty: ident]
    ),*) => {
        $(
            impl $op_trait for Object {
                type Output = EvalResult<Object>;
                fn $method(self) -> Self::Output {
                    use Object::*;
                    match self {
                        $(
                            $objects => $evaluation,
                        )*
                        o => return EvalResult::Err(EvaluationError::UnsupportedPrefixOperation(
                            PrefixOperator::$op_err_ty, o.r#type()
                        ))
                    }.into()
                }
            }
        )*
    };
}

// Rust macros are super awesome!!
impl_infix_op![
    [ops::Add => add : {
        (Int(v1), Int(v2)) => Object::from(v1 + v2),
        (Float(v1), Float(v2)) => Object::from(v1 + v2),
        (String(v1), String(v2)) => Object::from(std::string::String::from(&*v1) + &*v2)
    }, Add],

    [ops::Sub => sub : {
        (Int(v1), Int(v2)) => Object::from(v1 - v2),
        (Float(v1), Float(v2)) => Object::from(v1 - v2)
    }, Subtract],

    [ops::Mul => mul : {
        (Int(v1), Int(v2)) => Object::from(v1 * v2),
        (Float(v1), Float(v2)) => Object::from(v1 * v2)
    }, Multiply],

    [ops::Div => div : {
        (Int(v1), Int(v2)) => Object::from(v1 / v2),
        (Float(v1), Float(v2)) => Object::from(v1 / v2)
    }, Divide],

    [ops::Rem => rem : {
        (Int(v1), Int(v2)) => Object::from(v1 % v2),
        (Float(v1), Float(v2)) => Object::from(v1 % v2)
    }, Remainder]
];

impl_prefix_op![
    [ops::Neg => neg : {
        Int(v) => Object::from(-v),
        Float(v) => Object::from(-v)
    }, Neg],

    [ops::Not => not : {
        Boolean(v) => Object::from(!v)
    }, Not]
];

impl cmp::PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Int(v1), Object::Int(v2)) => v1.partial_cmp(v2),
            (Object::Float(v1), Object::Float(v2)) => v1.partial_cmp(v2),
            (Object::Char(v1), Object::Char(v2)) => v1.partial_cmp(v2),
            _ => None,
        }
    }
}

impl cmp::PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use Object as O;

        match (self, other) {
            (O::Int(v1), O::Int(v2)) => *v1 == *v2,
            (O::Float(v1), O::Float(v2)) => *v1 == *v2,
            (O::String(v1), O::String(v2)) => *v1 == *v2,
            (O::Char(v1), O::Char(v2)) => *v1 == *v2,
            (O::Boolean(v1), O::Boolean(v2)) => *v1 == *v2,
            (O::Type(v1), O::Type(v2)) => *v1 == *v2,
            (O::Address(v1), O::Address(v2)) => *v1 == *v2,
            (O::Null, O::Null) => true,
            _ => false,
        }
    }
}
