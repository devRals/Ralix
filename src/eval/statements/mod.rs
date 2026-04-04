use crate::{EvalResult, Evaluator, Expression, Object, Statement, try_eval_result};

mod assignment;
mod binding;

impl Evaluator<'_> {
    pub fn evaluate_statement(&mut self, stmt: Statement) -> EvalResult<Object> {
        match stmt {
            Statement::Binding(binding) => self.evaluate_binding(binding.ident, binding.value),
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::Assign { left, value } => self.evaluate_assignment_statement(left, value),
            Statement::Return(expr) => {
                let val = if let Some(e) = expr {
                    Some(try_eval_result!(self.evaluate_expression(e)))
                } else {
                    None
                };
                EvalResult::Return(val)
            }
            // Type checker special statement
            Statement::Alias { ident, ty } => self.evaluate_binding(ident, Expression::Type(ty)),
            Statement::Get {
                file_module_path,
                imported_items,
                path_names,
            } => {
                println!(
                    "{}",
                    Statement::Get {
                        path_names,
                        file_module_path,
                        imported_items
                    }
                );
                EvalResult::NoValue
            }
        }
    }
}
