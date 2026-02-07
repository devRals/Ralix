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
        if self.symbol_table.resolve_ref(ident).is_some() {
            if is_constant {
                return Err(CheckerError::AlreadyDefinedConstant(ident.clone()));
            } else {
                return Err(CheckerError::AlreadyDefined(ident.clone()));
            }
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

        let mut value_ty = self.check_expression(value)?;
        if let Some(ty_a) = type_annotation {
            infer_generics(ty_a, &mut value_ty);
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

fn infer_generics(type_annotation: &Type, value_ty: &mut Type) {
    match (value_ty, type_annotation) {
        (Type::Array(got), Type::Array(expected)) => {
            if let Type::Unknown = &**got {
                **got = *expected.clone()
            }
        }
        (
            Type::HashMap { key, value },
            Type::HashMap {
                key: expected_k,
                value: expected_v,
            },
        ) => {
            if let Type::Unknown = &**key {
                **key = *expected_k.clone()
            }
            if let Type::Unknown = &**value {
                **value = *expected_v.clone()
            }
        }
        _ => {}
    }
}
