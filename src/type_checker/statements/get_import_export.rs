use std::path::Path;

use crate::{
    CheckerResult, Statement, TypeChecker, TypeCheckerDiagnostic,
    expressions::{Identifier, ImportedItem},
};

// FIX: even though TypeChecker has a module cache it still causes stack overflow
// when two modules import each other

impl TypeChecker<'_> {
    pub fn check_get_statement(
        &mut self,
        module_path: &Path,
        imported_items: &[ImportedItem],
        module_name: Identifier,
    ) -> CheckerResult<()> {
        let module = match self.module_cache.get(&module_path.to_path_buf()) {
            Some(cached) => cached,
            None => {
                let module = self.parse_using_module_cache(module_path, module_name.clone())?;
                // it's O(1) so uhh... Gosh i feel like an idiot. Maybe cuz i am?
                self.module_cache.insert(module_path.to_path_buf(), module);
                self.module_cache.get(module_path).expect("Design error")
            }
        };

        let mut unknown_imports = Vec::new();
        for item in imported_items {
            for (exported_item_name, exported_item_type) in &module.exports {
                if &item.name == exported_item_name {
                    if let Some(as_name) = item.as_naming.clone() {
                        self.symbol_table
                            .define(as_name, exported_item_type.clone(), true)
                    } else {
                        self.symbol_table.define(
                            item.name.clone(),
                            exported_item_type.clone(),
                            true,
                        )
                    }
                } else {
                    unknown_imports.push(item.name.clone());
                }
            }
        }

        if unknown_imports.is_empty() {
            Ok(())
        } else {
            Err(TypeCheckerDiagnostic::UnknownImport(
                module_name,
                unknown_imports,
            ))
        }
    }

    pub fn check_out_statement(&mut self, stmt: &Statement) -> CheckerResult<()> {
        let (name, ty) = match stmt {
            Statement::Binding(binding) if binding.is_constant => (
                binding.ident.clone(),
                binding
                    .type_annotation
                    .clone()
                    .unwrap_or(self.check_expression(&binding.value)?),
            ),
            Statement::Alias { ident, ty } => (ident.clone(), ty.clone()),
            stmt => return Err(TypeCheckerDiagnostic::CannotExport(Box::new(stmt.clone()))),
        };

        self.self_module.exports.insert(name, ty);
        Ok(())
    }
}
