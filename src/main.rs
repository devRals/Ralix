use clap::Parser;
use ralix::cli::{Cli, Commands, commands};

fn main() -> color_eyre::Result<()> {
    let error_builder = color_eyre::config::HookBuilder::default().display_location_section(false);
    error_builder.install()?;
    let app = Cli::parse();

    Ok(match app.command {
        Commands::Run(run_args) => commands::run::run(run_args),
        Commands::Repl(repl_args) => commands::repl::run(repl_args),
        Commands::Ast(ast_args) => commands::ast::run(ast_args),
        Commands::Meow => commands::meow::run(),
    }?)
}
