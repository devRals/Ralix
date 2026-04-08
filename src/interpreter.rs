use std::{
    fs, io,
    path::{self, Path, PathBuf},
};

use crate::{
    EvalResult, Evaluator, Lexer, Parser, Program, ProgramCheckError, ProgramParseError,
    TypeChecker, Value, eval, expressions::Identifier, object, symbol_table::SymbolTable,
    type_checker,
};

#[derive(Debug, Default)]
pub struct Interpreter {
    pub symbol_table: SymbolTable,
    pub eval_ctx: EvalContext,
    pub tc_ctx: TypeCheckerContext,
    pub working_directory: PathBuf,
}

#[derive(Debug, Default)]
pub struct EvalContext {
    pub env: object::Environment,
    pub heap: object::Heap,
    pub module_cache: eval::ModuleCache,
}

#[derive(Debug, Default)]
pub struct TypeCheckerContext {
    /// Holds module names
    pub module_trace: Vec<Identifier>,
    pub module_cache: type_checker::ModuleCache,
}

pub enum ExecuteResult {
    IoError(io::Error),
    ParserError(ProgramParseError),
    CheckError(ProgramCheckError),
    EvalResult(EvalResult<Value>),
}

impl Interpreter {
    pub fn new(working_directory: impl AsRef<Path>) -> io::Result<Self> {
        let working_directory = {
            let wd = working_directory.as_ref();
            if !wd.is_absolute() {
                path::absolute(wd)?
            } else {
                wd.to_path_buf()
            }
        };
        Ok(Interpreter {
            working_directory,
            ..Default::default()
        })
    }

    pub fn execute_from_source(&mut self, source: &str) -> ExecuteResult {
        let program = match self.parse(source) {
            Ok(program_ast) => program_ast,
            Err(parse_error) => return ExecuteResult::ParserError(parse_error),
        };

        if let Err(check_error) = self.check(&program) {
            return ExecuteResult::CheckError(check_error);
        }

        ExecuteResult::EvalResult(self.execute(program))
    }

    pub fn execute_file_module<P: AsRef<Path>>(&mut self, module_path: P) -> ExecuteResult {
        let module_source = match fs::read_to_string(module_path) {
            Ok(source) => source,
            Err(path_read_error) => return ExecuteResult::IoError(path_read_error),
        };

        self.execute_from_source(&module_source)
    }

    pub fn execute(&mut self, program: Program) -> EvalResult<Value> {
        let mut evaluator = Evaluator::new(eval::RuntimeContext {
            environment: &mut self.eval_ctx.env,
            heap: &mut self.eval_ctx.heap,
            module_cache: &mut self.eval_ctx.module_cache,
            self_module: object::module::Module::default(),
        });

        evaluator.evaluate_program(program)
    }

    pub fn check(&mut self, program: &Program) -> Result<(), ProgramCheckError> {
        let mut type_checker = TypeChecker::new(
            &mut self.symbol_table,
            &mut self.tc_ctx.module_cache,
            &mut self.tc_ctx.module_trace,
        );
        type_checker.check_program(program)
    }

    /// # Panics
    /// - If `path::absolute(working_directory);` fails
    pub fn parse(&mut self, source: &str) -> Result<Program, ProgramParseError> {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer, &mut self.symbol_table, &self.working_directory);
        parser.parse_program()
    }
}
