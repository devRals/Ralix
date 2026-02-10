use std::{fmt::Display, rc::Rc};

use crate::{Environment, Heap, Object, Program};

mod context;
mod error;
mod expressions;
mod impls;
mod statements;

pub use context::*;
pub use error::*;

#[macro_export]
macro_rules! try_eval_result {
    ( $eval_result: expr ) => {{
        let value = $eval_result;
        match value {
            EvalResult::NoValue | EvalResult::Err(_) => return value,
            EvalResult::Return(_) => return value,
            EvalResult::Value(v) => v,
        }
    }};
}

impl From<Object> for EvalResult<Object> {
    fn from(val: Object) -> Self {
        EvalResult::Value(val)
    }
}

impl From<Option<Object>> for EvalResult<Object> {
    fn from(value: Option<Object>) -> Self {
        match value {
            Some(val) => Self::Value(val),
            None => Self::NoValue,
        }
    }
}

impl Object {
    pub fn copy_bits(&self) -> Option<Object> {
        match self {
            Object::Int(v) => Object::from(*v),
            Object::Char(v) => Object::from(*v),
            Object::Float(v) => Object::from(*v),
            Object::Boolean(v) => Object::from(*v),
            Object::Type(v) => Object::Type(v.clone()),
            Object::String(v) => Object::String(
                Rc::clone(v), /* Only copies the pointer that points the orijinal string value */
            ),
            Object::Address(addr) => Object::Address(addr.clone()),
            Object::Null => Object::NULL,
            Object::Function(func) => Object::Function(func.clone()),
            _ => return None,
        }
        .into()
    }
}

pub struct Evaluator<'env> {
    ctx: Context<'env>,
}

impl<'env> Evaluator<'env> {
    pub const fn new(environment: &'env mut Environment, heap: &'env mut Heap) -> Self {
        Evaluator {
            ctx: Context { environment, heap },
        }
    }
}

impl Evaluator<'_> {
    pub fn evaluate_program(&mut self, program: Program) -> EvalResult<Object> {
        let mut result = EvalResult::NoValue;

        for s in program.statements {
            result = self.evaluate_statement(s);

            match &result {
                EvalResult::Value(_) => {}
                EvalResult::NoValue => {}
                EvalResult::Return(obj) => match obj.clone() {
                    Some(o) => return EvalResult::Value(o),
                    None => return EvalResult::NoValue,
                },
                EvalResult::Err(err) => self.panic(err),
            }
        }

        result
    }

    pub fn panic<M: Display>(&self, msg: M) -> ! {
        panic!("\x1b[91mPanic!\x1b[0m: {msg}")
    }
}
