use crate::{
    EvalResult, Evaluator, Expression, Object, expressions::FunctionParameter, types::Type,
};

impl Evaluator<'_> {
    pub fn evaluate_function_expression(
        &mut self,
        parameters: Vec<FunctionParameter>,
        body: Expression,
        return_type: Type,
    ) -> EvalResult<Object> {
        EvalResult::Value(Object::Function {
            parameters,
            return_type,
            body,
            // I don't think this is a good idea but im super
            // dumb and lazy to find a solution for this
            env: self.ctx.environment.clone(),
        })
    }
}
