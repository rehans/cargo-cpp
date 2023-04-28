// Copyright(c) 2023 rehans.

// This declaration will look for a file named `project.rs` and will
// insert its contents inside a module named `project` under this scope
mod cpp_new;

use clap::{Parser, Subcommand};
use log::info;
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
        #[arg(short, long, help = "e.g. my_company_name")]
        domain_name: String,
        #[arg(short, long, default_value = "my_target")]
        target_name: String,
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Cli::parse();
    match args.command {
        Commands::New {
            domain_name,
            target_name,
            output_dir,
        } => {
            info!("Domain: {domain_name:#?}");
            info!("Target: {target_name:#?}");
            info!("Output: {output_dir:#?}");

            cpp_new::NewOptions::new(domain_name, target_name, output_dir).gen();
        }
    }

    Ok(())
}
