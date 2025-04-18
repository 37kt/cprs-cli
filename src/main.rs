mod parse_toml;
mod search_toml;

use clap::{Parser, Subcommand};

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
    let cargo_toml = parse_toml::parse_toml(&cargo_toml_path)?;

    let compete_toml_path = search_toml::search_toml_path("compete.toml")?;
    let compete_toml = parse_toml::parse_toml(&compete_toml_path)?;

    println!("Cargo.toml: {:?}", cargo_toml);
    println!("compete.toml: {:?}", compete_toml);

    Ok(())
}
