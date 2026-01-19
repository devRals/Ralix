use crate::{Environment, Node, NodeV, Object, Program, SymbolTable};

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
    pub fn copy(self) -> EvalResult<Object> {
        if matches!(
            self,
            Object::Boolean(_) | Object::Int(_) | Object::Char(_) | Object::Float(_)
        ) {
            EvalResult::Value(self)
        } else {
            EvalResult::Err(EvaluationError::UnsupportedCopyType(self.object_type()))
        }
    }
}

pub struct Evaluator<'env> {
    ctx: Context<'env>,
}

impl<'env> Evaluator<'env> {
    pub const fn new(
        symbol_table: &'env mut SymbolTable,
        environment: &'env mut Environment,
    ) -> Self {
        Evaluator {
            ctx: Context {
                environment,
                symbol_table,
            },
        }
    }
}

impl Evaluator<'_> {
    pub fn evaluate(&mut self, node: impl Node) -> EvalResult<Object> {
        match node.downcast() {
            NodeV::Program(program) => self.evaluate_program(program),
            NodeV::Statement(stmt) => self.evaluate_statement(stmt),
            NodeV::Expression(expr) => self.evaluate_expression(expr),
        }
    }
    pub fn evaluate_program(&mut self, program: Program) -> EvalResult<Object> {
        let mut result = EvalResult::NoValue;

        for s in program.statements {
            result = self.evaluate(s);

            match &result {
                EvalResult::Value(_) => {}
                EvalResult::NoValue => {}
                EvalResult::Err(err) => return EvalResult::Err(err.clone()),
            }
        }

        result
    }
}
