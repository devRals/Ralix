pub mod ast;
pub mod cli;
pub mod eval;
pub mod lexer;
pub mod logger;
pub mod object;
pub mod parser;
pub mod symbol_table;
pub mod type_checker;

pub use ast::*;
pub use cli::*;
pub use eval::*;
pub use lexer::*;
pub use logger::*;
pub use object::*;
pub use parser::*;
pub use symbol_table::*;
pub use type_checker::*;

#[derive(Debug)]
pub enum ExecuteError {
    ParserError(ProgramParseError),
    TypeCheckerError(ProgramCheckError),
    PanicError(EvaluationError),
}

impl std::error::Error for ExecuteError {}
impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParserError(e) => e.fmt(f),
            Self::TypeCheckerError(e) => e.fmt(f),
            Self::PanicError(e) => e.fmt(f),
        }
    }
}

pub fn execute_file_module<P: AsRef<std::path::Path>>(
    source_file_path: P,
) -> std::io::Result<Option<Object>> {
    let source = std::fs::read_to_string(source_file_path)?;
    execute(&source)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))
}

pub fn execute(source: &str) -> Result<Option<Object>, ExecuteError> {
    let mut env = Environment::default();
    execute_with_env(source, &mut env)
}

pub fn execute_with_env(
    source: &str,
    env: &mut Environment,
) -> Result<Option<Object>, ExecuteError> {
    let program = parse(source)?;

    let mut evaluator = Evaluator::new(env);
    match evaluator.evaluate_program(program) {
        EvalResult::Err(e) => Err(ExecuteError::PanicError(e)),
        EvalResult::Value(o) => Ok(Some(o)),
        EvalResult::Return(o) => Ok(o),
        EvalResult::NoValue => Ok(None),
    }
}

pub fn parse(source: &str) -> Result<Program, ExecuteError> {
    let mut st = SymbolTable::default();
    parse_with_symbol_table(source, &mut st)
}

pub fn parse_with_symbol_table(
    source: &str,
    symbol_table: &mut SymbolTable,
) -> Result<Program, ExecuteError> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer, symbol_table);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => return Err(ExecuteError::ParserError(e)),
    };

    let mut checker = TypeChecker::with_symbol_table(symbol_table);
    if let Err(e) = checker.check_program(&program) {
        return Err(ExecuteError::TypeCheckerError(e));
    }

    Ok(program)
}
