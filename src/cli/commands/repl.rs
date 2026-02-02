use crate::repl::Repl;
use clap::Args;
use ratatui::DefaultTerminal;

#[derive(Args, Debug)]
pub struct REPLArguments {
    #[arg(long)]
    legacy: bool,
}

pub fn run(_args: REPLArguments) -> color_eyre::Result<()> {
    ratatui::run(run_repl)
}

fn run_repl(term: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut repl = Repl::new();

    repl.run(term)
}
