use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker,
    expressions::FunctionParameter,
    types::{FunctionParameterType, Type},
};

impl TypeChecker<'_> {
    pub fn check_function_expression(
        &mut self,
        f_parameters: &[FunctionParameter],
        body: &Expression,
        return_type: &Type,
    ) -> CheckerResult<Type> {
        self.symbol_table.enter_scope();
        self.enter_function(return_type.clone());
        let mut parameters = Vec::new();

        for param in f_parameters {
            self.symbol_table.define(
                param.name.clone(),
                param.type_def.clone(),
                param.is_constant,
            );
            parameters.push(FunctionParameterType {
                is_constant: param.is_constant,
                ty: param.type_def.clone(),
            })
        }

        let body_ty = self.check_expression(body)?;

        if !body_ty.satisfies(return_type) {
            return Err(CheckerError::Unsatisfied(body_ty, return_type.clone()));
        }

        self.symbol_table.leave_scope();
        self.leave_function();

        Ok(Type::Function {
            parameters,
            return_type: Box::new(return_type.clone()),
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

                for (arg, param) in argument_types.iter().zip(parameters.iter().map(|p| &p.ty)) {
                    if !arg.satisfies(param) {
                        return Err(CheckerError::Unsatisfied(arg.clone(), param.clone()));
                    }
                }

                Ok(*return_type)
            }
            Type::AsValue(ty) => {
                if argument_types.len() != 1 {
                    return Err(CheckerError::MismatchedArgumentCount(1, arguments.len()));
                }
                let first_arg_ty = argument_types.first().unwrap();

                if first_arg_ty == &*ty {
                    return Ok(*ty);
                }

                // I might be stupid
                let is_available_for_cast = matches!(
                    (&*ty, first_arg_ty),
                    (Type::Int, Type::Float)
                        | (Type::Float, Type::Int)
                        | (Type::String, Type::Char | Type::Int | Type::Float)
                );

                if !is_available_for_cast {
                    return Err(CheckerError::UnavailableForCast(*ty, first_arg_ty.clone()));
                }

                Ok(*ty)
            }
            t => Err(CheckerError::CannotBeCalled(t)),
        }
    }
}
