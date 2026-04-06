use std::{
    io::{BufRead, BufReader, Result, Write, stderr, stdin, stdout},
    path::Path,
};

use color_eyre::owo_colors::OwoColorize;

use crate::{EvalResult, Heap, Interpreter, Object};

const CONTINUATION_PROMPT: &str = "...";
const PROMPT: &str = ">>>";

enum ReplResult {
    Success,
    Error,
    Init,
}

impl ReplResult {
    fn prompt(&self) -> String {
        match self {
            ReplResult::Init => PROMPT.cyan().to_string(),
            ReplResult::Error => PROMPT.bright_red().to_string(),
            ReplResult::Success => PROMPT.bright_green().to_string(),
        }
    }
}

pub struct Repl {
    buf: String,
    last_repl_result: ReplResult,
}

impl Repl {
    pub const fn new() -> Repl {
        Repl {
            buf: String::new(),
            last_repl_result: ReplResult::Init,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut reader = BufReader::new(stdin());
        let mut out = stdout();
        let mut interpreter = Interpreter::new(Path::new("."))?;

        let user = env!("USER").cyan();
        let bin_name = env!("CARGO_PKG_NAME").magenta();
        let version = format!(
            "{}.{}.{}",
            env!("CARGO_PKG_VERSION_MAJOR").yellow(),
            env!("CARGO_PKG_VERSION_MINOR").yellow(),
            env!("CARGO_PKG_VERSION_PATCH").yellow()
        );
        let home_page = env!("CARGO_PKG_HOMEPAGE").green();

        writeln!(
            out,
            "Welcome {user}!\n{bin_name} {version}\nCheckout {home_page} if you wanna learn more about {bin_name}\n"
        )?;

        loop {
            write!(out, "{} ", self.last_repl_result.prompt())?;
            out.flush()?;
            reader.read_line(&mut self.buf)?;
            self.buf.pop(); // remove "\n"

            match self.buf.as_str() {
                "st" => {
                    writeln!(out, "{:#?}\n", interpreter.symbol_table)?;
                    self.buf.clear();
                    continue;
                }
                "heap" => {
                    writeln!(out, "{:#?}\n", interpreter.eval_ctx.heap)?;
                    self.buf.clear();
                    continue;
                }
                "tc_cache" => {
                    writeln!(out, "{:#?}\n", interpreter.tc_ctx.module_cache)?;
                    self.buf.clear();
                    continue;
                }
                "env" => {
                    writeln!(out, "{:#?}\n", interpreter.eval_ctx.env)?;
                    self.buf.clear();
                    continue;
                }
                "help" => {
                    let msg = format!(
                        r#"You can get help from {home_page}
Available Keywords are:
    st: Debug print the `SymbolTable`
    heap: Debug print the `Heap`
    tc_cache: Debug print the TypeChecker::module_cache
    env: Debug print the `Environment`
    help: Print this message
"#
                    );
                    writeln!(out, "{msg}\n")?;
                    self.buf.clear();
                    continue;
                }
                _ => {}
            };

            if !self.is_input_complete() {
                self.force_correct_input(&mut reader, &mut out)?;
            }

            let program = match interpreter.parse(&self.buf) {
                Ok(p) => p,
                Err(err) => {
                    self.write_errors_and_clear(err)?;
                    continue;
                }
            };

            if let Err(check_err) = interpreter.check(&program) {
                self.write_errors_and_clear(check_err)?;
                continue;
            }

            let value = interpreter.execute(program);
            writeln!(
                out,
                "{}\n",
                self.colorize_eval_result(&value, &interpreter.eval_ctx.heap)?
            )?;
            self.buf.clear();
        }
    }

    fn write_errors_and_clear(&mut self, err_msg: impl ToString) -> Result<()> {
        self.last_repl_result = ReplResult::Error;
        let mut err_out = stderr();
        writeln!(err_out, "{}", err_msg.to_string().red())?;
        self.buf.clear();
        Ok(())
    }

    fn colorize_eval_result(&mut self, r: &EvalResult<Object>, heap: &Heap) -> Result<String> {
        self.last_repl_result = ReplResult::Success;
        Ok(match r {
            EvalResult::Return(v) => match v {
                Some(v) => colorize_obj(v, heap),
                None => "".to_string(),
            },
            EvalResult::Value(v) => colorize_obj(v, heap),
            EvalResult::NoValue => "".to_string(),
            EvalResult::Err(err) => {
                self.write_errors_and_clear(err)?;
                "".black().to_string()
            }
        })
    }

    fn is_input_complete(&self) -> bool {
        let mut stack = Vec::new();
        for char in self.buf.chars() {
            match char {
                '(' | '{' | '[' => stack.push(char),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                _ => (),
            }
        }
        stack.is_empty()
    }

    fn force_correct_input(&mut self, r#in: &mut impl BufRead, out: &mut impl Write) -> Result<()> {
        write!(out, "{CONTINUATION_PROMPT} ")?;
        out.flush()?;
        r#in.read_line(&mut self.buf)?;

        if !self.is_input_complete() {
            self.force_correct_input(r#in, out)?;
        }

        Ok(())
    }
}

fn colorize_obj(obj: &Object, heap: &Heap) -> String {
    match obj {
        Object::Int(v) => v.bright_yellow().to_string(),
        Object::Float(v) => v.bright_yellow().to_string(),
        Object::Char(v) => format!("'{v}'").bright_yellow().to_string(),
        Object::String(v) => format!("\"{v}\"").bright_green().to_string(),
        Object::Boolean(v) => v.cyan().to_string(),
        Object::Type(v) => v.bright_yellow().to_string(),
        Object::Address(v) => v.to_string().bright_black().to_string(),
        Object::Null => "null".bright_black().to_string(),
        Object::Function(func) => func.white().to_string(),
        Object::Array(v) => format!(
            "[{}]",
            v.iter()
                .map(|x| colorize_obj(heap.read(x).unwrap(), heap))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Object::HashMap(v) => format!(
            "#{{ {} }}",
            v.iter()
                .map(|(_, (k, v))| format!(
                    "{}: {}",
                    colorize_obj(heap.read(k).unwrap(), heap),
                    colorize_obj(heap.read(v).unwrap(), heap)
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
