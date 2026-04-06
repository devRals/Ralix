use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    CheckerResult, Lexer, Parser, SymbolTable, TypeChecker, TypeCheckerDiagnostic,
    expressions::Identifier, type_checker::ModuleTrace, types::Type,
};

#[derive(Debug)]
pub enum ModuleState {
    Loading(ModuleTrace),
    Checked(Module),
}

pub type ModuleCache = HashMap<PathBuf, ModuleState>;

#[derive(Clone, Debug, Default)]
pub struct Module {
    pub name: Identifier,
    pub exports: HashMap<Identifier, Type>,
    pub trace: Vec<Identifier>,
}

impl TypeChecker<'_> {
    /// Doesn't adds to the cache because I'm fucking lost to the borrow checker again
    pub fn parse_using_module_cache<P: AsRef<Path>>(
        &mut self,
        module_path: P,
        module_name: Identifier,
    ) -> CheckerResult<Module> {
        let module_path = module_path.as_ref();
        let module_source = match fs::read_to_string(module_path) {
            Ok(src) => src,
            Err(read_error) => return Err(TypeCheckerDiagnostic::ModuleLoadError(read_error)),
        };

        let working_directory = module_path.parent().unwrap().to_path_buf();
        let mut st = SymbolTable::default();

        self.module_trace.push(module_name.clone());
        self.module_cache.insert(
            module_path.to_path_buf(),
            ModuleState::Loading(self.module_trace.clone()),
        );

        let mut parser = Parser::new(Lexer::new(&module_source), &mut st, &working_directory);
        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(parse_err) => return Err(TypeCheckerDiagnostic::ModuleParseError(parse_err)),
        };

        let mut type_checker = TypeChecker::new(&mut st, self.module_cache, self.module_trace);
        type_checker.self_module.name = module_name;
        if let Err(check_err) = type_checker.check_program(&program) {
            return Err(TypeCheckerDiagnostic::ModuleTypeCheckError(check_err));
        }

        // Binding to the cache outside of function
        Ok(type_checker.self_module)
    }
}
