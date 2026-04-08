use std::{fmt::Display, rc::Rc};

use crate::{Program, Value};

mod context;
mod error;
mod expressions;
mod module_cache;
mod statements;

pub use context::*;
pub use error::*;
pub use module_cache::*;

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

impl From<Value> for EvalResult<Value> {
    fn from(val: Value) -> Self {
        EvalResult::Value(val)
    }
}

impl From<Option<Value>> for EvalResult<Value> {
    fn from(value: Option<Value>) -> Self {
        match value {
            Some(val) => Self::Value(val),
            None => Self::NoValue,
        }
    }
}

impl Value {
    pub fn copy_bits(&self) -> Option<Value> {
        match self {
            Value::Int(v) => Value::from(*v),
            Value::Char(v) => Value::from(*v),
            Value::Float(v) => Value::from(*v),
            Value::Boolean(v) => Value::from(*v),
            Value::Type(v) => Value::Type(v.clone()),
            Value::String(v) => Value::String(
                Rc::clone(v), /* Only copies the pointer that points the orijinal string value */
            ),
            Value::Pointer(addr) => Value::Pointer(*addr),
            Value::Null => Value::NULL,
            Value::Function(func) => Value::Function(func.clone()),
            _ => return None,
        }
        .into()
    }
}

pub struct Evaluator<'ctx> {
    ctx: RuntimeContext<'ctx>,
}

impl<'ctx> Evaluator<'ctx> {
    pub const fn new(ctx: RuntimeContext<'ctx>) -> Self {
        Evaluator { ctx }
    }
}

impl Evaluator<'_> {
    pub fn evaluate_program(&mut self, program: Program) -> EvalResult<Value> {
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
                EvalResult::Err(err) => self.err(err),
            }
        }

        result
    }

    pub fn err<M: Display>(&self, msg: M) {
        eprintln!("\x1b[91mRuntime Error!\x1b[0m: {msg}");
    }
}
