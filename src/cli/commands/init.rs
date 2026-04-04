use std::io;

use clap::Args;

#[derive(Args, Debug)]
pub struct InitArguments {}

pub fn run(_args: InitArguments) -> io::Result<()> {
    println!("wip");
    Ok(())
}
