mod config;
mod search_toml;

use anyhow::Context;
use clap::{Parser, Subcommand};
use config::{Submit, load_config};
use liquid::{ParserBuilder, object};

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

    let problem_id = match cli.command {
        Commands::Submit { problem_id } => problem_id,
    };

    let cargo_toml_path = search_toml::search_toml_path("Cargo.toml")?;
    let cargo_toml = std::fs::read_to_string(&cargo_toml_path)?;

    let workspace_path = std::path::Path::new(&cargo_toml_path)
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let compete_toml_path = search_toml::search_toml_path("compete.toml")?;
    let compete_toml = std::fs::read_to_string(&compete_toml_path)?;

    let config = load_config(&cargo_toml, &compete_toml)?;

    let problem = config
        .problems
        .iter()
        .find(|p| p.name == problem_id || p.alias == problem_id)
        .context("problem not found")?;

    let globals = object!({
        "src_path": workspace_path + "/" + &problem.path,
        "contest": &config.contest,
        "bin_name": &problem.name,
        "bin_alias": &problem.alias,
    });

    match config.submit {
        Submit::File { path } => {
            let path = ParserBuilder::with_stdlib().build()?.parse(&path)?;
            let path = path.render(&globals)?;
            let content = std::fs::read_to_string(&path)?;
            println!("{}", content);
        }
        Submit::Command { args } => {
            let args = args
                .iter()
                .map(|arg| {
                    let arg = ParserBuilder::with_stdlib()
                        .build()
                        .unwrap()
                        .parse(arg)
                        .unwrap();
                    arg.render(&globals).unwrap()
                })
                .collect::<Vec<_>>();
            println!("{:?}", args);
        }
    }

    Ok(())
}
