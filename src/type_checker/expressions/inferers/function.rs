use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker, expressions::FunctionParameter,
    types::Type,
};

impl TypeChecker<'_> {
    pub fn check_function_expression(
        &mut self,
        parameters: &[FunctionParameter],
        body: &Expression,
        return_type: Type,
    ) -> CheckerResult<Type> {
        self.symbol_table.enter_scope();
        for (param_ty, param_name) in parameters {
            self.symbol_table
                .define(param_name.clone(), param_ty.clone());
        }

        let body_ty = self.check_expression(body)?;

        if !body_ty.satisfies(&return_type) {
            return Err(CheckerError::Unsatisfied(body_ty, return_type));
        }

        self.symbol_table.leave_scope();

        Ok(return_type)
    }
}
