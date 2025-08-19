use clap::{Parser, Subcommand};
use anyhow::Result;

mod types;
mod storage;
mod project;
mod commands;

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

        #[arg(long)]
        project: Option<String>,

        secret: String,
    },

    List {
        #[arg(long)]
        global: bool,

        #[arg(long)]
        project: Option<String>,
    },

    Load {
        #[arg(long)]
        global: bool,

        #[arg(long)]
        project: Option<String>,

        #[arg(long)]
        export: bool,
    },

    Remove {
        #[arg(long)]
        global: bool,

        #[arg(long)]
        project: Option<String>,

        key: String,
    },
    Local {
        project: Option<String>,
    },

    Which,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let storage = Storage::new()?;

    match cli.command {
        Commands::Add { global, project, secret } => {
            commands::add::execute(&storage, global, project, &secret)
        }
        Commands::List { global, project } => {
            commands::list::execute(&storage, global, project)
        }
        Commands::Load { global, project, export } => {
            commands::load::execute(&storage, global, project, export)
        }
        Commands::Remove { global, project, key } => {
            commands::remove::execute(&storage, global, project, &key)
        }
        Commands::Local { project } => {
            commands::local::execute(project)
        }

        Commands::Which => {
            match project::get_current_project()? {
                Some(config) => println!("{}", config.display()),
                None => println!("No project configured"),
            }
            Ok(())
        }
    }
}
