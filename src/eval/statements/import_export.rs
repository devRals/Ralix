use std::path::PathBuf;

use crate::{
    EvalResult, Evaluator, Expression, RuntimeError, Statement, Value,
    eval::ModuleState,
    expressions::{Identifier, ImportedItem},
};

impl Evaluator<'_> {
    pub fn evaluate_import_statement(
        &mut self,
        file_module_path: PathBuf,
        imported_items: Vec<ImportedItem>,
        module_name: Identifier,
    ) -> EvalResult<Value> {
        let module = match self.ctx.module_cache.get(&file_module_path) {
            Some(module_state) => match module_state {
                ModuleState::Loaded(m) => m,
                ModuleState::Loading => {
                    return EvalResult::Err(RuntimeError::ImportCycleDetected(module_name));
                }
            },
            None => {
                let module = match self.evaluate_module(file_module_path.clone()) {
                    Ok(m) => m,
                    Err(module_err) => {
                        return EvalResult::Err(RuntimeError::ModuleExecuteError(module_err));
                    }
                };
                self.ctx
                    .module_cache
                    .insert(file_module_path.clone(), ModuleState::Loaded(module));

                match self.ctx.module_cache.get(&file_module_path).unwrap() {
                    ModuleState::Loaded(m) => m,
                    _ => unreachable!(),
                }
            }
        };

        for item in imported_items {
            self.ctx.environment.define(
                item.as_naming.unwrap_or(item.name.clone()),
                module[item.name],
            );
        }

        EvalResult::NoValue
    }

    pub fn evaluate_export_statement(&mut self, stmt: Statement) -> EvalResult<Value> {
        match stmt {
            Statement::Binding(binding) => {
                let binding_result = self.evaluate_binding(binding.ident.clone(), binding.value);
                let addr = self.ctx.get_addr(&binding.ident).unwrap();
                self.ctx.self_module.export(binding.ident, addr);
                binding_result
            }
            Statement::Alias { ident, ty } => {
                let binding_result = self.evaluate_binding(ident.clone(), Expression::Type(ty));
                let addr = self.ctx.get_addr(&ident).unwrap();
                self.ctx.self_module.export(ident, addr);
                binding_result
            }
            _ => unreachable!(),
        }
    }
}
