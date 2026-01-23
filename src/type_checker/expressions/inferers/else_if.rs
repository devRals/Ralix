use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker, expressions::IfConsequence, types::Type,
};

type ElseConsequence = Expression;
impl TypeChecker<'_> {
    pub fn infer_if_else_expression(
        &mut self,
        consequences: &[IfConsequence],
        else_consequence: Option<&ElseConsequence>,
    ) -> CheckerResult<Type> {
        let mut is_first = true;
        let mut first_branch_ty = Type::Void;
        let mut is_nullable = false;

        for (condition, cons) in consequences {
            let condition_ty = self.check_expression(condition)?;
            let cons_ty = self.check_expression(cons)?;

            if !condition_ty.satisfies(&Type::Bool) {
                return Err(CheckerError::Unsatisfied(condition_ty, Type::Bool));
            }

            if is_first {
                if let Type::Null = &cons_ty {
                    first_branch_ty = Type::Null;
                    is_nullable = true;
                    continue;
                }

                is_first = false;
                first_branch_ty = Type::Nullable(Box::new(cons_ty));
                continue;
            }

            if let Type::Null = &cons_ty {
                is_nullable = true;
            }

            if !cons_ty.satisfies(&first_branch_ty) {
                return Err(CheckerError::IfBranchesUnsatisfied(
                    first_branch_ty,
                    cons_ty,
                ));
            }
        }

        if let Some(else_con) = else_consequence {
            let else_con_ty = self.check_expression(else_con)?;
            if !else_con_ty.satisfies(&first_branch_ty) {
                return Err(CheckerError::IfBranchesUnsatisfied(
                    first_branch_ty,
                    else_con_ty,
                ));
            }

            if let Type::Null = &else_con_ty {
                is_nullable = true
            }

            if !is_nullable {
                first_branch_ty = first_branch_ty.unwrap_nullable()
            }
        }

        Ok(first_branch_ty)
    }
}

mod tests {

    #[test]
    fn test_infer_if_expression() {
        use crate::{
            CheckerError::*, Expression, Lexer, Parser, SymbolTable, TypeChecker, types::Type::*,
        };

        let tests = [
            ("if true: 10", Ok(Nullable(Int.into()))),
            ("if true: 10 else: 20 ", Ok(Int)),
            ("if true: 10 else if true: 20", Ok(Nullable(Int.into()))),
            ("if true: 10 else if true: 20 else: 30", Ok(Int)),
            (
                "if true: 10 else if true: null else: 20",
                Ok(Nullable(Int.into())),
            ),
            (
                "if true: 10 else if true: 20 else: null",
                Ok(Nullable(Int.into())),
            ),
            (
                "if true: null else if true: 20 else: 10",
                Ok(Nullable(Int.into())),
            ),
            // --- Errors
            ("if 'a': 10", Err(Unsatisfied(Char, Bool))),
            (
                "if true: 10 else: 30.0",
                Err(IfBranchesUnsatisfied(Nullable(Int.into()), Float)),
            ),
        ];

        for (i, (input, expected_result)) in tests.into_iter().enumerate() {
            let mut st = SymbolTable::default();
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer, &mut st);
            let if_expr = parser
                .parse_if_expression()
                .unwrap_or_else(|err| panic!("{err}"));
            let mut tc = TypeChecker::with_symbol_table(&mut st);

            if let Expression::IfElse {
                consequences,
                else_consequence,
            } = if_expr
            {
                let tc_result =
                    tc.infer_if_else_expression(&consequences, else_consequence.as_deref());
                assert_eq!(
                    tc_result, expected_result,
                    "{i}. {tc_result:?} != {expected_result:?}"
                );
            } else {
                panic!("method not gived and if expression")
            }
        }
    }
}
