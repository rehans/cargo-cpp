mod project_gen;

use clap::{Parser, Subcommand};
use project_gen::generate::gen_project;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, about = "Generate a new project")]
    New {
        #[arg(short, long, help = "e.g. company name")]
        domain_name: Option<String>,
        #[arg(short, long, default_value = "my-target")]
        target_name: String,
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::New {
            domain_name: domain,
            target_name,
            output_dir,
        } => {
            let current_dir = std::env::current_dir().unwrap().clone();
            gen_project(
                domain.unwrap_or("".to_string()),
                target_name,
                output_dir.unwrap_or(current_dir),
            );
        }
    }

    Ok(())
}
