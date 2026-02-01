use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker,
    expressions::Identifier,
    types::{FunctionParameterType, Type},
};

impl TypeChecker<'_> {
    pub fn check_binding(
        &mut self,
        ident: &Identifier,
        type_annotation: Option<&Type>,
        value: &Expression,
        is_constant: bool,
    ) -> CheckerResult<()> {
        if let Some(v) = self.symbol_table.resolve_ref(ident)
            && v.is_constant
        {
            return Err(CheckerError::AlreadyDefinedConstant(ident.clone()));
        }

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
                    parameters: f_params
                        .iter()
                        .map(|param| FunctionParameterType {
                            ty: param.type_def.clone(),
                            is_constant: param.is_constant,
                        })
                        .collect(),
                    return_type: return_type.clone().into(),
                },
                is_constant,
            );
            self.check_function_expression(f_params, body, return_type)?;
            return Ok(());
        }

        let value_ty = self.check_expression(value)?;
        if let Some(ty_a) = type_annotation {
            if value_ty.satisfies(ty_a) {
                self.symbol_table
                    .define(ident.clone(), ty_a.clone(), is_constant);
                Ok(())
            } else {
                Err(CheckerError::Unsatisfied(value_ty, ty_a.clone()))
            }
        } else {
            self.symbol_table
                .define(ident.clone(), value_ty, is_constant);
            Ok(())
        }
    }
}
