use crate::repl::{REPLMode, Repl};
use clap::Args;
use std::io::{self, BufReader, stderr, stdin, stdout};

#[derive(Args, Debug)]
pub struct REPLArguments {
    /// The mode of REPL. Default is "eval"
    #[arg(long, short)]
    mode: Option<REPLMode>,
}

pub fn run(args: REPLArguments) -> io::Result<()> {
    let mut repl = Repl::new(
        BufReader::new(stdin()),
        stdout(),
        stderr(),
        args.mode.unwrap_or(REPLMode::Eval),
    );
    repl.run()
}
