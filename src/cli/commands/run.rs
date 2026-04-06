use clap::Args;
use std::{io, path::PathBuf};

use crate::{EvalResult, ExecuteResult, Interpreter};

#[derive(Args, Debug)]
pub struct RunArguments {
    file: Option<PathBuf>,
}

pub fn run(args: RunArguments) -> io::Result<()> {
    let mut interpreter = Interpreter::new(args.file.as_ref().unwrap().parent().unwrap())?;

    match args.file {
        Some(f) => match interpreter.execute_file_module(f) {
            ExecuteResult::IoError(io_error) => Err(io_error),
            ExecuteResult::ParserError(parse_error) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                parse_error.to_string(),
            )),
            ExecuteResult::CheckError(check_error) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                check_error.to_string(),
            )),
            ExecuteResult::EvalResult(result) => match result {
                EvalResult::Err(runtime_error) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    runtime_error.to_string(),
                )),
                _ => Ok(()),
            },
        },
        None => run_project(),
    }
}

fn run_project() -> io::Result<()> {
    todo!("Project folder implemention is coming soon")
}
