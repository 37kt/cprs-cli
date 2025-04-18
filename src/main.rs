mod config;
mod parse_toml;
mod search_toml;

use clap::{Parser, Subcommand};
use config::load_config;

#[derive(Parser)]
#[command(name = "cprs-cli", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Submit { problem_id: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Submit { problem_id } => {
            println!("submit: {}", problem_id);
        }
    }

    let cargo_toml_path = search_toml::search_toml_path("Cargo.toml")?;
    let cargo_toml = std::fs::read_to_string(cargo_toml_path)?;

    // let compete_toml_path = search_toml::search_toml_path("compete.toml")?;
    // let compete_toml = std::fs::read_to_string(compete_toml_path)?;

    let config = load_config(&cargo_toml)?;

    eprintln!("{:?}", &config);

    Ok(())
}
