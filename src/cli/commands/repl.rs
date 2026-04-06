use std::io;

use crate::repl::Repl;
use clap::Args;

#[derive(Args, Debug)]
pub struct REPLArguments {}

pub fn run(_args: REPLArguments) -> io::Result<()> {
    Repl::new().run()
}
