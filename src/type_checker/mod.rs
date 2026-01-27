mod error;
mod expressions;
mod statements;
pub use error::*;

use crate::{Program, SymbolTable, types::Type};

struct FunctionContext {
    return_type: Type,
}

pub struct TypeChecker<'st> {
    symbol_table: &'st mut SymbolTable,
    fn_stack: Vec<FunctionContext>,
}

impl<'st> TypeChecker<'st> {
    pub const fn with_symbol_table(symbol_table: &'st mut SymbolTable) -> Self {
        Self {
            symbol_table,
            fn_stack: Vec::new(),
        }
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

    pub fn enter_function(&mut self, fn_return_ty: Type) {
        self.fn_stack.push(FunctionContext {
            return_type: fn_return_ty,
        });
    }

    pub fn leave_function(&mut self) {
        self.fn_stack.pop();
    }

    pub fn current_fn_return_type(&self) -> &Type {
        self.fn_stack
            .last()
            .map(|stack| &stack.return_type)
            .unwrap_or(&Type::Void)
    }
}
