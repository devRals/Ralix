use std::io::{self, BufRead, Write};

use crate::{Environment, EvalResult, Evaluator, Lexer, Logger, Parser, SymbolTable, TypeChecker};

enum PromptResult {
    Error,
    Success,
    Init,
}

pub struct Repl<W: Write, EW: Write, R: BufRead> {
    err_out: EW,
    out: W,
    r#in: R,
    last_prompt_result: PromptResult,
}

const PROMPT: &str = ">>> ";
const _HELP_PROMPT: &str = "help> ";
const CLEAR: &str = "\x1b[0m";

impl<W: Write, EW: Write, R: BufRead> Repl<W, EW, R> {
    pub const fn new(r#in: R, out: W, err_out: EW) -> Self {
        Self {
            r#in,
            out,
            err_out,
            last_prompt_result: PromptResult::Init,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut buf = String::new();
        let mut symbol_table = SymbolTable::default();
        let mut error_logger = Logger::new(&mut self.err_out);
        let mut env = Environment::default();

        loop {
            write!(
                self.out,
                "{}{PROMPT}{CLEAR}",
                match self.last_prompt_result {
                    PromptResult::Error => "\x1b[91m",
                    PromptResult::Success => "\x1b[92m",
                    PromptResult::Init => "\x1b[94m",
                }
            )?;
            self.out.flush()?;
            self.r#in.read_line(&mut buf)?;
            buf.pop(); // Remove the '\n' character

            match buf.as_str() {
                "help" => {
                    println!("Help mode is coming soon");
                    buf.clear();
                    continue;
                }

                "env" => {
                    dbg!(&env);
                    buf.clear();
                    continue;
                }

                _ => {}
            }

            let lexer = Lexer::new(&buf);
            let mut parser = Parser::new(lexer, &mut symbol_table);
            let program = match parser.parse_program() {
                Ok(program) => program,
                Err(err) => {
                    self.last_prompt_result = PromptResult::Error;
                    error_logger.error(err)?;
                    buf.clear();
                    writeln!(self.out)?;
                    continue;
                }
            };

            let mut type_checker = TypeChecker::with_symbol_table(&mut symbol_table);
            if let Err(type_errors) = type_checker.check_program(&program) {
                self.last_prompt_result = PromptResult::Error;
                error_logger.error(type_errors)?;
                buf.clear();
                writeln!(self.out)?;
                continue;
            }

            let mut evaluator = Evaluator::new(&mut env);
            match evaluator.evaluate_program(program) {
                EvalResult::NoValue => Ok(()),
                EvalResult::Value(val) => writeln!(self.out, "{val}"),
                EvalResult::Err(err) => {
                    self.last_prompt_result = PromptResult::Error;
                    error_logger.error(err)?;
                    buf.clear();
                    writeln!(self.out)?;
                    continue;
                }
            }?;

            self.last_prompt_result = PromptResult::Success;
            writeln!(self.out)?;

            buf.clear();
        }
    }
}
