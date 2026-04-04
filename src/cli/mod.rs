use std::io;

use clap::{Parser, Subcommand};

pub mod commands;
pub mod repl;

/// Ralix command line interface
#[derive(Parser, Debug)]
#[command(name = "ralix", about = "Command line interface for ralix", version)]
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
    /// Initialize a ralix project in the specified directory
    Init(commands::init::InitArguments),
    /// A Cute, fluffy kitty :3
    Meow,
}

impl Cli {
    pub fn run(self) -> io::Result<()> {
        match self.command {
            Commands::Run(run_args) => commands::run::run(run_args),
            Commands::Repl(repl_args) => commands::repl::run(repl_args),
            Commands::Ast(ast_args) => commands::ast::run(ast_args),
            Commands::Init(init_args) => commands::init::run(init_args),
            Commands::Meow => commands::meow::run(),
        }
    }
}
