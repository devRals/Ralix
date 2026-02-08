use crate::repl::{Repl, legacy};
use clap::Args;
use ratatui::DefaultTerminal;

#[derive(Args, Debug)]
pub struct REPLArguments {
    #[arg(long)]
    /// Experimental terminal user interface
    tui: bool,
}

pub fn run(args: REPLArguments) -> color_eyre::Result<()> {
    if args.tui {
        ratatui::run(run_repl_tui)
    } else {
        Ok(legacy::Repl::new().run()?)
    }
}

fn run_repl_tui(term: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut repl = Repl::new();

    repl.run(term)
}
