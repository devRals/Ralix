use std::{fs, path::Path};

use crate::{
    CheckerError, CheckerResult, Lexer, Parser, SymbolTable, TypeChecker, expressions::ImportedItem,
};

impl TypeChecker<'_> {
    pub fn check_get_statement(
        &mut self,
        module_path: &Path,
        imported_items: &[ImportedItem],
    ) -> CheckerResult<()> {
        let module_source = match fs::read_to_string(module_path) {
            Ok(src) => src,
            Err(read_error) => return Err(CheckerError::ModuleLoadError(read_error)),
        };

        let working_directory = module_path.parent().unwrap().to_path_buf();
        let mut st = SymbolTable::default();

        let mut parser = Parser::new(
            Lexer::new(&module_source),
            &mut st,
            working_directory.clone(),
        );
        let program = match parser.parse_program() {
            Ok(program) => program,
            Err(parse_err) => return Err(CheckerError::ModuleParseError(parse_err)),
        };

        let mut type_checker = TypeChecker::with_symbol_table(&mut st, working_directory);
        if let Err(check_err) = type_checker.check_program(&program) {
            return Err(CheckerError::ModuleTypeCheckError(check_err));
        }

        println!("{:?}: {st:#?}\n", module_path);

        Ok(())
    }
}
