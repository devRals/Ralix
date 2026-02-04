use std::io::{BufRead, BufReader, Result, Write, stderr, stdin, stdout};

use color_eyre::owo_colors::OwoColorize;

use crate::{
    Environment, EvalResult, Evaluator, Object, SymbolTable, parse_with_symbol_table,
    repl::render::{CONTINUATION_PROMPT, PROMPT},
};

pub struct Repl {
    buf: String,
}

impl Repl {
    pub const fn new() -> Repl {
        Repl { buf: String::new() }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut reader = BufReader::new(stdin());
        let mut out = stdout();

        let mut env = Environment::default();
        let mut st = SymbolTable::default();

        loop {
            write!(out, "{PROMPT} ")?;
            out.flush()?;
            reader.read_line(&mut self.buf)?;

            if !self.is_input_complete() {
                self.force_correct_input(&mut reader, &mut out)?;
            }

            let program = match parse_with_symbol_table(&self.buf, &mut st) {
                Ok(p) => p,
                Err(err) => {
                    self.write_errors_and_clear(err)?;
                    continue;
                }
            };

            let mut evaluator = Evaluator::new(&mut env);
            let value = evaluator.evaluate_program(program);
            writeln!(out, "{}\n", self.colorize_eval_result(&value)?)?;
            self.buf.clear();
        }
    }

    fn write_errors_and_clear(&mut self, err_msg: impl ToString) -> Result<()> {
        let mut err_out = stderr();
        writeln!(err_out, "{}", err_msg.to_string().red())?;
        self.buf.clear();
        Ok(())
    }

    fn colorize_eval_result(&mut self, r: &EvalResult<Object>) -> Result<String> {
        let no_value = "No Value".bright_black().to_string();
        Ok(match r {
            EvalResult::Return(v) => match v {
                Some(v) => colorize_obj(v).to_string(),
                None => no_value,
            },
            EvalResult::Value(v) => colorize_obj(v).to_string(),
            EvalResult::NoValue => no_value,
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

fn colorize_obj(obj: &Object) -> String {
    match obj {
        Object::Int(v) => v.bright_yellow().to_string(),
        Object::Float(v) => v.bright_yellow().to_string(),
        Object::Char(v) => format!("'{v}'").bright_cyan().to_string(),
        Object::String(v) => format!("\"{v}\"").bright_green().to_string(),
        Object::Boolean(v) => v.cyan().to_string(),
        Object::Type(v) => v.bright_yellow().to_string(),
        Object::Address(v) => format!("<{v:?}>").bright_black().to_string(),
        Object::Null => "null".bright_black().to_string(),
        Object::Function(func) => func.white().to_string(),
        Object::Array(_) => obj.white().to_string(),
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
