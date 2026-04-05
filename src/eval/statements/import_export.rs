use std::path::PathBuf;

use crate::{
    EvalResult, Evaluator, Object, Statement,
    expressions::{Identifier, ImportedItem},
};

impl Evaluator<'_> {
    pub fn evaluate_import_statement(
        &mut self,
        file_module_path: PathBuf,
        imported_items: Vec<ImportedItem>,
        path_names: Vec<Identifier>,
        module_name: Identifier,
    ) -> EvalResult<Object> {
        println!(
            "{}",
            Statement::Get {
                module_name,
                path_names,
                file_module_path,
                imported_items
            }
        );
        EvalResult::NoValue
    }

    pub fn evaluate_export_statement(&mut self, stmt: Statement) -> EvalResult<Object> {
        println!("{}", Statement::Out(stmt.into()));
        EvalResult::NoValue
    }
}
