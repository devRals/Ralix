use crate::{CheckerError, CheckerResult, Expression, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn check_array_literal(&mut self, items: &[Expression]) -> CheckerResult<Type> {
        let mut is_nullable = false;
        let mut is_first = true;
        let mut ty = Type::Unknown;

        for i in items {
            let item_ty = self.check_expression(i)?;
            if is_first {
                if item_ty.is_nullish() {
                    is_nullable = true;
                    continue;
                }

                is_first = false;
                ty = item_ty.clone();
            }

            if item_ty.is_nullish() {
                is_nullable = true;
                continue;
            }

            if ty.includes_unknown() && !item_ty.is_nullish() {
                ty = item_ty;
                continue;
            }

            if !item_ty.satisfies(&ty) {
                return Err(CheckerError::ArrayHasMultipleDifferentType(ty, item_ty));
            }
        }

        Ok(Type::Array(if is_nullable {
            Type::Nullable(ty.into()).into()
        } else {
            ty.into()
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use Type::*;
    #[test]
    fn test_array_literal_infer() {
        let tests = [
            ("[1,2,3,4,5]", Ok(Array(Int.into()))),
            ("[2, null, 10]", Ok(Array(Nullable(Int.into()).into()))),
            (
                "[2, \"string\", 10]",
                Err(CheckerError::ArrayHasMultipleDifferentType(Int, String)),
            ),
            ("[]", Ok(Array(Unknown.into()))),
            ("[null, 'a']", Ok(Array(Nullable(Char.into()).into()))),
        ];

        for (i, (input, expected_result)) in tests.into_iter().enumerate() {
            let mut st = SymbolTable::default();
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer, &mut st);
            let arr = parser
                .parse_array_literal()
                .unwrap_or_else(|err| panic!("{err}"));
            let mut tc = TypeChecker::with_symbol_table(&mut st);

            if let Expression::Array { items } = arr {
                let tc_result = tc.check_array_literal(&items);
                assert_eq!(
                    tc_result, expected_result,
                    "{i}. {tc_result:?} != {expected_result:?}"
                );
            } else {
                panic!("method not gived an array expression")
            }
        }
    }
}
