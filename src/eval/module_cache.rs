use crate::{
    Environment, EvalResult, Evaluator, Lexer, Parser, RuntimeContext, SymbolTable, object::Module,
};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum ModuleState {
    Loading,
    Loaded(Module),
}

pub type ModuleCache = HashMap<PathBuf, ModuleState>;

impl Evaluator<'_> {
    pub fn evaluate_module(&mut self, module_path: PathBuf) -> io::Result<Module> {
        let module_source = fs::read_to_string(&module_path)?;
        let working_directory = module_path.parent().unwrap_or(Path::new("."));

        self.ctx
            .module_cache
            .insert(module_path.clone(), ModuleState::Loading);

        let mut st = SymbolTable::default();
        let program = match Parser::new(Lexer::new(&module_source), &mut st, working_directory)
            .parse_program()
        {
            Ok(p) => p,
            Err(parse_error) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    parse_error.to_string(),
                ));
            }
        };

        // skiping type checking during runtime

        let mut environment = Environment::default();
        let ctx = RuntimeContext {
            module_cache: self.ctx.module_cache,
            heap: self.ctx.heap,
            environment: &mut environment,
            self_module: Module::default(),
        };
        let mut evaluator = Evaluator::new(ctx);
        if let EvalResult::Err(runtime_error) = evaluator.evaluate_program(program) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                runtime_error.to_string(),
            ));
        }

        Ok(evaluator.ctx.self_module)
    }
}
