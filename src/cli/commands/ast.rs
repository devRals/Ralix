use clap::Args;
use std::{
    fs::{self, OpenOptions},
    io::{self, Error, ErrorKind, Write},
    path::PathBuf,
};

use crate::Interpreter;

#[derive(Args, Debug)]
pub struct AstArguments {
    /// File that holds the JSON value. If an another file exist with the same it'll be overwritten
    #[arg(long, short, value_name = "FILE")]
    output: Option<PathBuf>,

    #[arg(value_name = "FILE")]
    source_file: PathBuf,
}

pub fn run(args: AstArguments) -> io::Result<()> {
    let working_directory = args.source_file.clone().parent().unwrap().to_path_buf();
    let mut interpreter = Interpreter::new(working_directory)?;
    let source = fs::read_to_string(args.source_file)?;

    let parse_result = interpreter.parse(&source);
    let program_ast = match parse_result {
        Ok(p) => p,
        Err(err) => {
            return Err(Error::new(ErrorKind::InvalidData, err.to_string()));
        }
    };

    let program_ast_as_string = match serde_json::to_string_pretty(&program_ast) {
        Ok(source) => source,
        Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
    };

    match args.output {
        Some(output_file_path) => {
            let mut src_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(output_file_path)?;
            src_file.write(program_ast_as_string.as_bytes()).map(|_| ())
        }
        None => {
            println!("{program_ast_as_string}");
            Ok(())
        }
    }
}
