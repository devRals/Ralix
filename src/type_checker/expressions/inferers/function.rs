use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker, expressions::FunctionParameter,
    types::Type,
};

impl TypeChecker<'_> {
    pub fn check_function_expression(
        &mut self,
        f_parameters: &[FunctionParameter],
        body: &Expression,
        return_type: Type,
    ) -> CheckerResult<Type> {
        self.symbol_table.enter_scope();
        let mut parameters = Vec::new();

        for (param_ty, param_name) in f_parameters {
            self.symbol_table
                .define(param_name.clone(), param_ty.clone());
            parameters.push(param_ty.clone())
        }

        let body_ty = self.check_expression(body)?;

        if !body_ty.satisfies(&return_type) {
            return Err(CheckerError::Unsatisfied(body_ty, return_type));
        }

        self.symbol_table.leave_scope();

        Ok(Type::Function {
            parameters,
            return_type: Box::new(return_type),
        })
    }

    pub fn check_call_expression(
        &mut self,
        function: &Expression,
        arguments: &[Expression],
    ) -> CheckerResult<Type> {
        let func_ty = self.check_expression(function)?;
        let mut argument_types = Vec::new();

        for e in arguments {
            let arg_ty = self.check_expression(e)?;
            argument_types.push(arg_ty)
        }

        match func_ty {
            Type::Function {
                parameters,
                return_type,
            } => {
                if parameters.len() != argument_types.len() {
                    return Err(CheckerError::MismatchedArgumentCount(
                        parameters.len(),
                        argument_types.len(),
                    ));
                }

                for (arg, param) in argument_types.iter().zip(&parameters) {
                    if !arg.satisfies(param) {
                        return Err(CheckerError::Unsatisfied(arg.clone(), param.clone()));
                    }
                }

                Ok(*return_type)
            }
            t => Err(CheckerError::CannotBeCalled(t)),
        }
    }
}
