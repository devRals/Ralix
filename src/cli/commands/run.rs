use clap::Args;
use std::{io, path::PathBuf};

use crate::execute_file_module;

#[derive(Args, Debug)]
pub struct RunArguments {
    file: Option<PathBuf>,
}

pub fn run(args: RunArguments) -> io::Result<()> {
    match args.file {
        Some(f) => execute_file_module(&f).map(|_| ()),
        None => run_project(),
    }
}

fn run_project() -> io::Result<()> {
    todo!("Project folder implemention is coming soon")
}
