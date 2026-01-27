use crate::{
    Environment, EvalResult, Evaluator, Lexer, Logger, Parser, SymbolTable, Token, TypeChecker,
};
use clap::ValueEnum;
use std::io::{self, BufRead, Write};

#[derive(ValueEnum, Clone, Debug)]
pub enum REPLMode {
    Eval,
    Ast,
    Tokens,
}

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
    mode: REPLMode,
}

const PROMPT: &str = ">>> ";
const _HELP_PROMPT: &str = "help> ";
const CLEAR: &str = "\x1b[0m";

impl<W: Write, EW: Write, R: BufRead> Repl<W, EW, R> {
    pub const fn new(r#in: R, out: W, err_out: EW, mode: REPLMode) -> Self {
        Self {
            r#in,
            out,
            err_out,
            last_prompt_result: PromptResult::Init,
            mode,
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

                "st" | "symbol_table" => {
                    dbg!(&symbol_table);
                    buf.clear();
                    continue;
                }

                _ => {}
            }

            let lexer = Lexer::new(&buf);
            if let REPLMode::Tokens = self.mode {
                let tokens: Vec<Token> = lexer.collect();
                for (i, t) in tokens.iter().enumerate() {
                    writeln!(self.out, "{}. {t:?}", i + 1)?;
                }
                writeln!(self.out)?;
                buf.clear();
                continue;
            }

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

            if let REPLMode::Ast = self.mode {
                let ast_as_json = match serde_json::to_string_pretty(&program) {
                    Ok(res) => res,
                    Err(err) => return Err(io::Error::other(err)),
                };
                writeln!(self.out, "{ast_as_json}\n")?;
                buf.clear();
                continue;
            }

            let mut evaluator = Evaluator::new(&mut env);
            match evaluator.evaluate_program(program) {
                EvalResult::NoValue => Ok(()),
                EvalResult::Value(val) => writeln!(self.out, "{val}"),
                EvalResult::Return(val) => match val {
                    Some(o) => writeln!(self.out, "{o}"),
                    None => Ok(()),
                },
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
