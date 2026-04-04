use crate::{CheckerResult, Statement, TypeChecker, types::Type};

mod assignment;
mod binding;
mod get;
mod r#return;

impl TypeChecker<'_> {
    pub fn check_statement(&mut self, stmt: &Statement) -> CheckerResult<Option<Type>> {
        match stmt {
            Statement::Binding(binding) => self
                .check_binding(
                    &binding.ident,
                    binding.type_annotation.as_ref(),
                    &binding.value,
                    binding.is_constant,
                )
                .map(|_| None),
            Statement::Expression(expr) => self.check_expression(expr).map(Some),
            Statement::Assign { left, value } => {
                self.check_assignment_statement(left, value).map(|_| None)
            }
            Statement::Return(expr) => self.check_return_statement(expr.as_ref()).map(|_| None),
            Statement::Get {
                file_module_path,
                imported_items,
                path_names: _,
            } => self
                .check_get_statement(file_module_path, imported_items)
                .map(|_| None),
            Statement::Alias { .. } => Ok(None),
        }
    }
}
