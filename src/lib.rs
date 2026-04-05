pub mod ast;
pub mod cli;
pub mod eval;
pub mod lexer;
pub mod logger;
pub mod object;
pub mod parser;
pub mod symbol_table;
pub mod type_checker;

use std::{
    io,
    path::{self, Path},
};

pub(crate) use ast::*;
pub(crate) use cli::*;
pub(crate) use eval::*;
pub(crate) use lexer::*;
pub(crate) use object::*;
pub(crate) use parser::*;
pub(crate) use symbol_table::*;
pub(crate) use type_checker::*;

#[derive(Debug)]
pub enum ExecuteErrorBase {
    IoError(io::Error),
    ParserError(ProgramParseError),
    TypeCheckerError(ProgramCheckError),
    RuntimeError(EvaluationError),
}

pub type ExecuteError = Box<ExecuteErrorBase>;

impl std::error::Error for ExecuteErrorBase {}
impl std::fmt::Display for ExecuteErrorBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(io_err) => io_err.fmt(f),
            Self::ParserError(e) => e.fmt(f),
            Self::TypeCheckerError(e) => e.fmt(f),
            Self::RuntimeError(e) => e.fmt(f),
        }
    }
}

pub const DIRECTORY_INDEX_MODULE_NAME: &str = "package";
pub const RALIX_VALID_EXTENSIONS: &[&str] = &[".rl", ".rlx", ".ralix"];

pub fn execute_file_module<SrcP: AsRef<Path>, WdP: AsRef<Path>>(
    source_file_path: SrcP,
    working_directory: WdP,
) -> Result<Option<Object>, ExecuteError> {
    let source = match std::fs::read_to_string(&source_file_path) {
        Ok(src) => src,
        Err(path_err) => return Err(ExecuteErrorBase::IoError(path_err).into()),
    };
    execute(&source, working_directory)
}

pub fn execute<P: AsRef<Path>>(
    source: &str,
    working_directory: P,
) -> Result<Option<Object>, ExecuteError> {
    let mut env = Environment::default();
    let mut heap = Heap::new();
    let ctx = Context {
        environment: &mut env,
        heap: &mut heap,
    };
    execute_with_context(source, ctx, working_directory)
}

pub fn execute_with_context<P: AsRef<Path>>(
    source: &str,
    ctx: Context,
    working_directory: P,
) -> Result<Option<Object>, ExecuteError> {
    let program = parse(source, working_directory)?;

    let mut evaluator = Evaluator::new(ctx);
    match evaluator.evaluate_program(program) {
        EvalResult::Err(e) => Err(ExecuteErrorBase::RuntimeError(e).into()),
        EvalResult::Value(o) => Ok(Some(o)),
        EvalResult::Return(o) => Ok(o),
        EvalResult::NoValue => Ok(None),
    }
}

pub fn parse<P: AsRef<Path>>(source: &str, working_directory: P) -> Result<Program, ExecuteError> {
    let mut st = SymbolTable::default();
    let mut module_cache = type_checker::ModuleCache::new();
    parse_with_symbol_table(source, &mut st, working_directory, &mut module_cache)
}

pub fn parse_with_symbol_table<P: AsRef<Path>>(
    source: &str,
    symbol_table: &mut SymbolTable,
    working_directory: P,
    module_cache: &mut type_checker::ModuleCache,
) -> Result<Program, ExecuteError> {
    let wd = match path::absolute(working_directory) {
        Ok(path) => path,
        Err(path_error) => return Err(ExecuteErrorBase::IoError(path_error).into()),
    };

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer, symbol_table, wd);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(parse_error) => {
            return Err(ExecuteErrorBase::ParserError(parse_error).into());
        }
    };

    let mut checker = TypeChecker::with_symbol_table(symbol_table, module_cache);
    if let Err(check_error) = checker.check_program(&program) {
        return Err(ExecuteErrorBase::TypeCheckerError(check_error).into());
    }

    Ok(program)
}
