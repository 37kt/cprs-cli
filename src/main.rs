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

#[derive(Debug, Parser)]
struct Args {
    name: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Submit { problem_id } => {
            println!("submit: {}", problem_id);
        }
    }
}
