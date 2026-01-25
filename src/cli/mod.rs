use clap::{ColorChoice, Parser, Subcommand};

pub mod commands;
pub mod repl;

/// Ralix is a interpreter that combines the
/// familiarity of C, Rust, Javascript syntax with the flexibility
/// of optional semicolons and high-level type-system features.
#[derive(Parser, Debug)]
#[command(
    name = "ralix",
    version,
    about,
    long_about = None,
    color = ColorChoice::Always
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run a project/source file
    Run(commands::run::RunArguments),
    /// Read-Evaluate Print Loop
    Repl(commands::repl::REPLArguments),
    /// Print/Dump `Program` AST in json format
    Ast(commands::ast::AstArguments),
    /// A Cute, fluffy kitty :3
    Meow,
}
