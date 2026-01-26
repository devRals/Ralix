use crate::{
    EvalResult, EvaluationError, Evaluator, Expression, FunctionEnvironment, Object,
    expressions::FunctionParameter, try_eval_result, types::Type,
};

impl Evaluator<'_> {
    pub fn evaluate_function_expression(
        &mut self,
        parameters: Vec<FunctionParameter>,
        body: Expression,
        return_type: Type,
    ) -> EvalResult<Object> {
        let function = Object::new_function(
            parameters,
            return_type,
            body,
            // WARN: Im not sure it has to be like this.
            // Im only cloning the current scope
            FunctionEnvironment {
                items: self.ctx.environment.current_scope(),
            },
        );

        EvalResult::Value(Object::Function(function))
    }

    pub fn evaluate_call_expression(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
    ) -> EvalResult<Object> {
        let func = match try_eval_result!(self.evaluate_expression(function)) {
            Object::Function(func) => func,
            o => return EvalResult::Err(EvaluationError::IsNotAFunction(o.r#type())),
        };

        self.ctx.enter_scope();
        self.ctx.environment.extend_from(&func.env.items);

        for (arg, (_, param)) in arguments.into_iter().zip(func.parameters.clone()) {
            let arg_val = try_eval_result!(self.evaluate_expression(arg));
            self.ctx.define(param, arg_val);
        }

        let result = try_eval_result!(self.evaluate_expression(func.body.clone()));
        self.ctx.leave_scope();

        EvalResult::Value(result)
    }
}
