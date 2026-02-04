use crate::repl::{Repl, legacy};
use clap::Args;
use ratatui::DefaultTerminal;

#[derive(Args, Debug)]
pub struct REPLArguments {
    #[arg(long)]
    legacy: bool,
}

pub fn run(args: REPLArguments) -> color_eyre::Result<()> {
    if args.legacy {
        Ok(legacy::Repl::new().run()?)
    } else {
        ratatui::run(run_repl_tui)
    }
}

fn run_repl_tui(term: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut repl = Repl::new();

    repl.run(term)
}
