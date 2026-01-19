use crate::expressions::{InfixOperator, PrefixOperator};
use std::ops;

use crate::{EvalResult, EvaluationError, Object};

macro_rules! impl_infix_op {
    ($(
        [$op_trait: ident $method: ident : {$(
            $objects: pat => $evaluation: expr
        ),*}, $op_err_ty: ident]
    ),*) => {
        $(
            impl ops::$op_trait for Object {
                type Output = EvalResult<Object>;
                fn $method(self, other: Self) -> Self::Output {
                    use Object::*;
                    match (self, other) {
                        $(
                            $objects => $evaluation,
                        )*
                        (o1, o2) => return EvalResult::Err(EvaluationError::UnsupportedInfixOperation(
                            o1.object_type(), InfixOperator::$op_err_ty, o2.object_type()
                        ))
                    }.into()
                }
            }
        )*
    };
}

macro_rules! impl_prefix_op {
    ($(
        [$op_trait: ident $method: ident : {$(
            $objects: pat => $evaluation: expr
        ),*}, $op_err_ty: ident]
    ),*) => {
        $(
            impl ops::$op_trait for Object {
                type Output = EvalResult<Object>;
                fn $method(self) -> Self::Output {
                    use Object::*;
                    match self {
                        $(
                            $objects => $evaluation,
                        )*
                        o => return EvalResult::Err(EvaluationError::UnsupportedPrefixOperation(
                            PrefixOperator::$op_err_ty, o.object_type()
                        ))
                    }.into()
                }
            }
        )*
    };
}

// Rust macros are super awesome!!
impl_infix_op![
    [Add add : {
        (Int(v1), Int(v2)) => Object::from(v1 + v2),
        (Float(v1), Float(v2)) => Object::from(v1 + v2),
        (String(v1), String(v2)) => Object::from(std::string::String::from(&*v1) + &*v2)
    }, Add],

    [Sub sub : {
        (Int(v1), Int(v2)) => Object::from(v1 - v2),
        (Float(v1), Float(v2)) => Object::from(v1 - v2)
    }, Subtract],

    [Mul mul : {
        (Int(v1), Int(v2)) => Object::from(v1 * v2),
        (Float(v1), Float(v2)) => Object::from(v1 * v2)
    }, Multiply],

    [Div div : {
        (Int(v1), Int(v2)) => Object::from(v1 / v2),
        (Float(v1), Float(v2)) => Object::from(v1 / v2)
    }, Divide],

    [Rem rem : {
        (Int(v1), Int(v2)) => Object::from(v1 % v2),
        (Float(v1), Float(v2)) => Object::from(v1 % v2)
    }, Remainder]
];

impl_prefix_op![
    [Neg neg : {
        Int(v) => Object::from(-v),
        Float(v) => Object::from(-v)
    }, Neg],

    [Not not : {
        Boolean(v) => Object::from(!v)
    }, Not]
];
