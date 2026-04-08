use crate::{
    EvalResult, Evaluator, Expression, FunctionEnvironment, RuntimeError, Value,
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
    ) -> EvalResult<Value> {
        let function = Value::new_function(
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

        EvalResult::Value(Value::Function(function))
    }

    pub fn evaluate_call_expression(
        &mut self,
        function: Expression,
        arguments: Vec<Expression>,
    ) -> EvalResult<Value> {
        let func = match try_eval_result!(self.evaluate_expression(function)) {
            Value::Function(func) => func,
            Value::Type(ty) => return self.evaluate_type_casting(ty, arguments[0].clone()),
            o => return EvalResult::Err(RuntimeError::IsNotAFunction(o.r#type(self.ctx.heap))),
        };

        self.ctx.enter_scope();
        self.ctx.environment.extend_from(&func.env.items);

        for (arg, param) in arguments.into_iter().zip(func.parameters.clone()) {
            let param_name = param.name.clone();
            let arg_val = try_eval_result!(self.evaluate_expression(arg));
            self.ctx.define(param_name, arg_val);
        }

        let result = self.evaluate_expression(func.body.clone());
        // BUG: Leaving scope removes the parameters from  env but not from heap
        self.ctx.leave_scope();

        if matches!(func.return_type, Type::Void) {
            EvalResult::NoValue
        } else {
            unwrap_return_value(result)
        }
    }
}

fn unwrap_return_value(result: EvalResult<Value>) -> EvalResult<Value> {
    match result {
        // `into` converts the type `Option<Object>` into
        // EvalResult::Value(inner) if Some(inner)
        // EvalResult::NoValue if None
        EvalResult::Return(v) => v.into(),
        _ => result,
    }
}
