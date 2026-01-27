use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker, expressions::Identifier, types::Type,
};

impl TypeChecker<'_> {
    pub fn check_binding(
        &mut self,
        ident: &Identifier,
        type_annotation: Option<&Type>,
        value: &Expression,
    ) -> CheckerResult<()> {
        // this allows recursive functions
        if let Expression::Function {
            parameters: f_params,
            return_type,
            body,
        } = value
        {
            self.symbol_table.define(
                ident.clone(),
                Type::Function {
                    parameters: f_params.iter().map(|(t, _)| t.clone()).collect(),
                    return_type: return_type.clone().into(),
                },
            );
            self.check_function_expression(f_params, body, return_type)?;
            return Ok(());
        }

        let value_ty = self.check_expression(value)?;
        if let Some(ty_a) = type_annotation {
            if value_ty.satisfies(ty_a) {
                self.symbol_table.define(ident.clone(), ty_a.clone());
                Ok(())
            } else {
                Err(CheckerError::Unsatisfied(value_ty, ty_a.clone()))
            }
        } else {
            self.symbol_table.define(ident.clone(), value_ty);
            Ok(())
        }
    }
}
