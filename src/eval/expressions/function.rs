use crate::{
    EvalResult, EvaluationError, Evaluator, Expression, FunctionEnvironment, Object,
    expressions::FunctionParameter,
    try_eval_result,
    types::{Type, TypeVarId},
};

impl Evaluator<'_> {
    pub fn evaluate_function_expression(
        &mut self,
        parameters: Vec<FunctionParameter>,
        body: Expression,
        return_type: Type,
        generics: Vec<TypeVarId>,
    ) -> EvalResult<Object> {
        let function = Object::new_function(
            parameters,
            return_type,
            body,
            // WARN: Im not sure it has to be like this.
            // Im only cloning the current scope
            FunctionEnvironment {
                items: self.ctx.environment.current_items(),
            },
            generics,
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
            Object::Type(ty) => return self.evaluate_type_casting(ty, arguments[0].clone()),
            o => return EvalResult::Err(EvaluationError::IsNotAFunction(o.r#type())),
        };

        self.ctx.enter_scope();
        self.ctx.environment.extend_from(&func.env.items);

        for (arg, param) in arguments.into_iter().zip(func.parameters.clone()) {
            let param_name = param.name.clone();
            let arg_val = try_eval_result!(self.evaluate_expression(arg));
            self.ctx.define(param_name, arg_val);
        }

        let result = self.evaluate_expression(func.body.clone());
        self.ctx.leave_scope();

        if matches!(func.return_type, Type::Void) {
            EvalResult::NoValue
        } else {
            unwrap_return_value(result)
        }
    }
}

fn unwrap_return_value(result: EvalResult<Object>) -> EvalResult<Object> {
    match result {
        // `into` converts the type `Option<Object>` into
        // EvalResult::Value(inner) if Some(inner)
        // EvalResult::NoValue if None
        EvalResult::Return(v) => v.into(),
        _ => result,
    }
}
