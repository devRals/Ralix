use crate::{CheckerError, CheckerResult, TypeChecker, expressions::HashMapItem, types::Type};

impl TypeChecker<'_> {
    pub fn check_hashmap_literal(&mut self, items: &[HashMapItem]) -> CheckerResult<Type> {
        let (mut key, mut value) = (Type::Unknown, Type::Unknown);
        let mut is_nullable = false;
        let (mut key_is_first, mut value_is_first) = (true, true);

        for map_item in items {
            let (key_ty, value_ty) = (
                self.check_expression(&map_item.key)?,
                self.check_expression(&map_item.value)?,
            );

            if !key_ty.is_hashable() {
                return Err(CheckerError::CannotBeHashed(key_ty));
            } else if key_is_first {
                key = key_ty;
                key_is_first = false;
            } else if !key_ty.satisfies(&key) {
                return Err(CheckerError::HashMaphHasMultipleDifferentKeyTypes(
                    key, key_ty,
                ));
            }

            if value_is_first {
                if value_ty.is_nullish() {
                    is_nullable = true;
                    continue;
                }

                value_is_first = false;
                value = value_ty.clone()
            }

            if value_ty.is_nullish() {
                is_nullable = true;
                continue;
            }

            if value.is(&Type::Unknown) && !value_ty.is_nullish() {
                value = value_ty;
                continue;
            }

            if !value_ty.satisfies(&value) {
                return Err(CheckerError::HashMaphHasMultipleDifferentValueTypes(
                    value, value_ty,
                ));
            }
        }

        Ok(Type::HashMap {
            key: key.into(),
            value: if is_nullable {
                Type::Nullable(value.into()).into()
            } else {
                value.into()
            },
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        CheckerError::*, Expression, Lexer, Parser, SymbolTable, TypeChecker, types::Type::*,
    };

    #[test]
    fn test_hash_literal_infer() {
        let tests = [
            (
                r#" #{ "meow": 10 } "#,
                Ok(HashMap {
                    key: String.into(),
                    value: Int.into(),
                }),
            ),
            (
                r#" #{ 0: "zero", 1: "one", 2: "two" } "#,
                Ok(HashMap {
                    key: Int.into(),
                    value: String.into(),
                }),
            ),
            (
                r#" #{ "five": 5.0, "unknown": null } "#,
                Ok(HashMap {
                    key: String.into(),
                    value: Nullable(Float.into()).into(),
                }),
            ),
            (
                r#" #{ 10: null, 11: 30 } "#,
                Ok(HashMap {
                    key: Int.into(),
                    value: Nullable(Int.into()).into(),
                }),
            ),
            (
                r#" #{ 40: null, "owo": 30 } "#,
                Err(HashMaphHasMultipleDifferentKeyTypes(Int, String)),
            ),
            (
                r#" #{ 2: "two", 3: 30 } "#,
                Err(HashMaphHasMultipleDifferentValueTypes(String, Int)),
            ),
            (
                "#{}",
                Ok(HashMap {
                    key: Unknown.into(),
                    value: Unknown.into(),
                }),
            ),
        ];

        for (i, (input, expected_result)) in tests.into_iter().enumerate() {
            let mut st = SymbolTable::default();
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer, &mut st);
            let map = parser
                .parse_hashmap_literal()
                .unwrap_or_else(|err| panic!("{err}"));

            let mut tc = TypeChecker::with_symbol_table(&mut st);
            if let Expression::HashMap { items } = map {
                let tc_result = tc.check_hashmap_literal(&items);
                assert_eq!(
                    tc_result, expected_result,
                    "{i}. {tc_result:?} != {expected_result:?}"
                );
            } else {
                panic!("method not gived an hashmap literal expression")
            }
        }
    }
}
