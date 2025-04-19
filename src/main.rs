mod config;
mod search_toml;

use std::process::Command;

use anyhow::Context;
use arboard::Clipboard;
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
        "src_path": workspace_path.clone() + "/" + &problem.path,
        "contest": &config.contest,
        "bin_name": &problem.name,
        "bin_alias": &problem.alias,
    });

    match config.submit {
        Submit::File { path } => {
            let path = ParserBuilder::with_stdlib().build()?.parse(&path)?;
            let path = path.render(&globals)?;
            let content = std::fs::read_to_string(&path)?;
            eprintln!("content: {}", content);
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(content)?;
            println!("copied to clipboard.");
        }
        Submit::Command { args } => {
            let args = args
                .iter()
                .map(|arg| -> anyhow::Result<String> {
                    let arg = ParserBuilder::with_stdlib().build()?.parse(arg)?;
                    Ok(arg.render(&globals)?)
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            let output = Command::new(&args[0])
                .args(&args[1..])
                .current_dir(&workspace_path)
                .output()
                .context("failed to run command")?;

            if !output.status.success() {
                eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
            } else {
                let content = String::from_utf8_lossy(&output.stdout);
                let mut clipboard = Clipboard::new()?;
                clipboard.set_text(content)?;
                println!("copied to clipboard (status: {})", output.status);
            }
        }
    }

    println!("opening {}", problem.url);
    open::that(&problem.url)?;

    Ok(())
}
