use clap::Parser;
use ralix::cli::Cli;

fn main() -> color_eyre::Result<()> {
    let error_builder = color_eyre::config::HookBuilder::default().display_location_section(false);
    error_builder.install()?;
    let cli = Cli::parse();

    Ok(cli.run()?)
}
