use clap::Args;
use std::{io, path::PathBuf};

use crate::{ExecuteErrorBase, execute_file_module};

#[derive(Args, Debug)]
pub struct RunArguments {
    file: Option<PathBuf>,
}

pub fn run(args: RunArguments) -> io::Result<()> {
    match args.file {
        Some(f) => execute_file_module(&f, PathBuf::from("."))
            .map(|_| ())
            .map_err(|err| match *err {
                ExecuteErrorBase::IoError(e) => e,
                ExecuteErrorBase::ParserError(e) => {
                    io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
                }
                ExecuteErrorBase::TypeCheckerError(e) => {
                    io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
                }
                ExecuteErrorBase::RuntimeError(e) => {
                    io::Error::new(io::ErrorKind::InvalidData, e.to_string())
                }
            }),
        None => run_project(),
    }
}

fn run_project() -> io::Result<()> {
    todo!("Project folder implemention is coming soon")
}
