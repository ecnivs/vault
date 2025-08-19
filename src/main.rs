use clap::{Parser, Subcommand};
use anyhow::Result;

mod types;
mod storage;
mod commands;

use types::SecretScope;
use storage::Storage;

#[derive(Parser)]
#[command(name = "vault")]
#[command(about = "A CLI tool for managing secrets and environment variables")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(long)]
        global: bool,

        #[arg(long, required_unless_present = "global")]
        project: Option<String>,

        #[arg(long, required_unless_present = "global")]
        env: Option<String>,

        secret: String,
    },

    List {
        #[arg(long)]
        global: bool,

        #[arg(long, required_unless_present = "global")]
        project: Option<String>,

        #[arg(long, required_unless_present = "global")]
        env: Option<String>,
    },

    Load {
        #[arg(long)]
        global: bool,

        #[arg(long, required_unless_present = "global")]
        project: Option<String>,

        #[arg(long, required_unless_present = "global")]
        env: Option<String>,

        #[arg(long)]
        export: bool,
    },

    Remove {
        #[arg(long)]
        global: bool,

        #[arg(long, required_unless_present = "global")]
        project: Option<String>,

        #[arg(long, required_unless_present = "global")]
        env: Option<String>,

        key: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let storage = Storage::new()?;

    match cli.command {
        Commands::Add { global, project, env, secret } => {
            let scope = if global {
                SecretScope::global()
            } else {
                match (project, env) {
                    (Some(p), Some(e)) => SecretScope::project(p, e),
                    _ => {
                        eprintln!("Error: Project and environment required when not using --global");
                        std::process::exit(1);
                    }
                }
            };

            commands::add::execute(&storage, &scope, &secret)
        }

        Commands::List { global, project, env } => {
            let scope = if global {
                SecretScope::global()
            } else {
                match (project, env) {
                    (Some(p), Some(e)) => SecretScope::project(p, e),
                    _ => {
                        eprintln!("Error: Project and environment required when not using --global");
                        std::process::exit(1);
                    }
                }
            };

            commands::list::execute(&storage, &scope)
        }

        Commands::Load { global, project, env, export } => {
            let scope = if global {
                SecretScope::global()
            } else {
                match (project, env) {
                    (Some(p), Some(e)) => SecretScope::project(p, e),
                    _ => {
                        eprintln!("Error: Project and environment required when not using --global");
                        std::process::exit(1);
                    }
                }
            };

            commands::load::execute(&storage, &scope, export)
        }

        Commands::Remove { global, project, env, key } => {
            let scope = if global {
                SecretScope::global()
            } else {
                match (project, env) {
                    (Some(p), Some(e)) => SecretScope::project(p, e),
                    _ => {
                        eprintln!("Error: Project and environment required when not using --global");
                        std::process::exit(1);
                    }
                }
            };

            commands::remove::execute(&storage, &scope, &key)
        }
    }
}
