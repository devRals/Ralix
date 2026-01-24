use clap::Parser;
use ralix::{Commands, cli::Cli, commands};

fn main() -> anyhow::Result<()> {
    let app = Cli::parse();

    Ok(match app.command {
        Commands::Run(run_args) => commands::run::run(run_args),
        Commands::Repl => commands::repl::run(),
        Commands::Ast(ast_args) => commands::ast::run(ast_args),
        Commands::Meow => commands::meow::run(),
    }?)
}
