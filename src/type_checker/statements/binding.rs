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
            generics,
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
                    generics: generics.clone(),
                },
                is_constant,
            );
            self.check_function_expression(f_params, body, return_type, generics)?;
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

pub fn infer_generics(type_annotation: &Type, mut value_ty: &mut Type) {
    match (&mut value_ty, type_annotation) {
        (Type::Array(got), Type::Array(expected)) => {
            infer_generics(expected, got);
        }
        (
            Type::HashMap {
                key: got_k,
                value: got_v,
            },
            Type::HashMap {
                key: expected_k,
                value: expected_v,
            },
        ) => {
            infer_generics(expected_k, got_k);
            infer_generics(expected_v, got_v);
        }
        (Type::Nullable(got), Type::Nullable(expected)) => infer_generics(expected, got),
        (Type::AsValue(got), Type::AsValue(expected)) => infer_generics(expected, got),
        (Type::Addr(got), Type::Addr(expected)) => infer_generics(expected, got),
        (
            Type::Function {
                parameters: got_p,
                return_type: got_rt,
                generics: _,
            },
            Type::Function {
                parameters: expected_p,
                return_type: expected_rt,
                generics: _,
            },
        ) => {
            for (e, g) in expected_p.iter().zip(got_p) {
                infer_generics(&e.ty, &mut g.ty);
            }
            infer_generics(expected_rt, got_rt);
        }

        (got, expected) => {
            if !expected.is(&Type::Unknown) && got.is(&Type::Unknown) {
                **got = expected.clone()
            }
        }
    }
}
