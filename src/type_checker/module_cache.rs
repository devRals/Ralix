use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    CheckerResult, Lexer, Parser, SymbolTable, TypeChecker, TypeCheckerDiagnostic,
    expressions::Identifier, types::Type,
};

pub type ModuleCache = HashMap<PathBuf, Module>;

#[derive(Clone, Debug, Default)]
pub struct Module {
    pub name: Identifier,
    pub exports: HashMap<Identifier, Type>,
}

impl TypeChecker<'_> {
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

        let mut parser = Parser::new(Lexer::new(&module_source), &mut st, working_directory);
        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(parse_err) => return Err(TypeCheckerDiagnostic::ModuleParseError(parse_err)),
        };

        let mut type_checker = TypeChecker::with_symbol_table(&mut st, self.module_cache);
        type_checker.self_module.name = module_name;
        if let Err(check_err) = type_checker.check_program(&program) {
            return Err(TypeCheckerDiagnostic::ModuleTypeCheckError(check_err));
        }

        Ok(type_checker.self_module)
    }
}
