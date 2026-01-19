pub struct TypeChecker<'st> {
    symbol_table: &'st mut SymbolTable,
}

mod error;
mod expressions;
mod statements;
pub use error::*;

use crate::{Program, SymbolTable};

impl<'st> TypeChecker<'st> {
    pub const fn with_symbol_table(symbol_table: &'st mut SymbolTable) -> Self {
        Self { symbol_table }
    }
}

impl TypeChecker<'_> {
    pub fn check_program(&mut self, program: &Program) -> Result<(), ProgramCheckError> {
        let mut errors = Vec::new();

        for stmt in &program.statements {
            match self.check_statement(stmt) {
                Ok(_) => {}
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ProgramCheckError::new(errors))
        }
    }
}
