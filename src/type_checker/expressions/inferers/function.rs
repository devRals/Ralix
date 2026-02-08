use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker,
    expressions::FunctionParameter,
    types::{FunctionParameterType, Type, TypeVarId},
};

impl TypeChecker<'_> {
    pub fn check_function_expression(
        &mut self,
        f_parameters: &[FunctionParameter],
        body: &Expression,
        return_type: &Type,
        generics: &[TypeVarId],
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
            self.symbol_table.leave_scope();
            self.leave_function();
            return Err(CheckerError::Unsatisfied(body_ty, return_type.clone()));
        }

        self.symbol_table.leave_scope();
        self.leave_function();

        Ok(Type::Function {
            parameters,
            return_type: Box::new(return_type.clone()),
            generics: generics.into(),
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
                generics: _,
            } => {
                if parameters.len() != argument_types.len() {
                    return Err(CheckerError::MismatchedArgumentCount(
                        parameters.len(),
                        argument_types.len(),
                    ));
                }

                for (arg, param) in argument_types.iter().zip(parameters) {
                    let param = self.unify_typevar(param.ty.clone(), arg.clone())?;

                    if !arg.satisfies(&param) {
                        return Err(CheckerError::Unsatisfied(arg.clone(), param));
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

#[cfg(test)]
mod test {
    use crate::{Lexer, Parser, Statement, SymbolTable};

    impl TypeChecker<'_> {
        /// idx: 0 - Default - HashMap Key - Function
        /// idx: 1 - HashMap Value - Function Parameter index or return type if its greater
        pub(crate) fn resolve_typevar_from_complex_data_type(&self, ty: Type, idx: usize) -> Type {
            match ty {
                Type::TypeVar(id) => {
                    if let Some(bound) = self.typevar_bindings.get(&id) {
                        self.resolve_typevar(bound.clone())
                    } else {
                        Type::TypeVar(id)
                    }
                }
                Type::Array(ty) => self.resolve_typevar(*ty),
                Type::Nullable(ty) => self.resolve_typevar(*ty),
                Type::AsValue(ty) => self.resolve_typevar(*ty),
                Type::Addr(ty) => self.resolve_typevar(*ty),
                Type::HashMap { key, value } => {
                    if idx > 0 {
                        self.resolve_typevar(*value)
                    } else {
                        self.resolve_typevar(*key)
                    }
                }
                Type::Function {
                    parameters,
                    return_type,
                    ..
                } => {
                    if idx > parameters.len() {
                        self.resolve_typevar(*return_type)
                    } else {
                        self.resolve_typevar(parameters[idx].ty.clone())
                    }
                }
                other => other,
            }
        }
    }

    use super::*;
    use Type::*;
    #[test]
    fn test_function_generics() {
        let tests: [(&str, &[Type]); _] = [
            ("fn x[T](T y) -> T: y; x(2)", &[Int]),
            (
                "fn x[T](arr[T] y) -> T?: y[0]; x([\"a\", \"b\"])",
                &[String],
            ),
            (
                "fn x[T](map[int, T] y) -> T?: y[0]; x(#{ 0: [2.0, 3.14] })",
                &[Array(Float.into())],
            ),
            (
                "fn insert[V](map[str, V] values, str key, V val): { values[key] = val }; insert(#{ \"\": false }, \"a\", true)",
                &[Bool],
            ),
            (
                "fn x[T, U](T y, U z): {}; x(['a'], #{ 40: 40.0 })",
                &[
                    Array(Char.into()),
                    HashMap {
                        key: Int.into(),
                        value: Float.into(),
                    },
                ],
            ),
            (
                "fn x[T](T? y) -> T?: y; x({ str? a = \"a\"; a })",
                &[String],
            ),
            ("fn x[T](T* y) -> T*: y; x({ char a = 'a'; &a })", &[Char]),
        ];

        for (i, (input, expected_generic)) in tests.into_iter().enumerate() {
            let lexer = Lexer::new(input);
            let mut st = SymbolTable::default();
            let mut parser = Parser::new(lexer, &mut st);
            let p = parser
                .parse_program()
                .unwrap_or_else(|err| panic!("{i}. {err}"));
            let mut tc = TypeChecker::with_symbol_table(&mut st);

            let (ident, type_annotation, value, is_constant) = match &p[0] {
                Statement::Binding {
                    ident,
                    type_annotation,
                    value,
                    is_constant,
                } => (ident, type_annotation, value, is_constant),
                s => panic!("{s} is not expected"),
            };

            tc.check_binding(ident, type_annotation.as_ref(), value, *is_constant)
                .unwrap_or_else(|err| panic!("{i}. {err}"));

            let (fn_params, arguments, generics_count) = match &p[p.statements.len() - 1] {
                Statement::Expression(expr) => match expr {
                    Expression::Call {
                        function,
                        arguments,
                    } => match tc
                        .check_expression(function)
                        .unwrap_or_else(|err| panic!("{i}. {err}"))
                    {
                        Function {
                            parameters,
                            generics,
                            ..
                        } => (
                            parameters,
                            arguments
                                .iter()
                                .map(|a| {
                                    tc.check_expression(a)
                                        .unwrap_or_else(|err| panic!("{i}. {err}"))
                                })
                                .collect::<Vec<_>>(),
                            generics.len(),
                        ),
                        t => panic!("{i}. {t} is not expected"),
                    },
                    e => panic!("{i}. {e} is not expected"),
                },
                s => panic!("{i}. {s} is not expected"),
            };
            assert_eq!(fn_params.len(), arguments.len());

            // BUG: This is a stupid check.
            for (j, (p, arg)) in fn_params.iter().zip(arguments).enumerate() {
                tc.unify_typevar(p.ty.clone(), arg.clone())
                    .unwrap_or_else(|err| panic!("{i} - {j}: {err}"));

                let resolved = tc.resolve_typevar_from_complex_data_type(p.ty.clone(), 1);

                if j == generics_count {
                    break;
                }

                assert_eq!(
                    resolved, expected_generic[j],
                    "{i}. {resolved} != {}",
                    expected_generic[j]
                );
            }
        }
    }
}
