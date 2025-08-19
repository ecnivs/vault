use crate::types::ProjectConfig;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{PathBuf};

const VAULT_VERSION_FILE: &str = ".vault-version";

pub fn find_project_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;

    loop {
        let vault_file = current_dir.join(VAULT_VERSION_FILE);
        if vault_file.exists() {
            return Some(current_dir);
        }

        if !current_dir.pop() {
            break;
        }
    }

    None
}

pub fn get_current_project() -> Result<Option<ProjectConfig>> {
    if let Some(project_root) = find_project_root() {
        let vault_file = project_root.join(VAULT_VERSION_FILE);
        let content = fs::read_to_string(&vault_file)
            .context("Failed to read .vault-version file")?;

        let trimmed = content.trim();
        return Ok(Some(ProjectConfig::new(trimmed.to_string())));
    }

    Ok(None)
}

pub fn set_current_project(config: &ProjectConfig) -> Result<()> {
    let current_dir = env::current_dir()
        .context("Failed to get current directory")?;

    let vault_file = current_dir.join(VAULT_VERSION_FILE);
    let content = config.display();

    fs::write(&vault_file, content)
        .context("Failed to write .vault-version file")?;

    println!("Set project to: {}", config.display());
    println!("Created: {}", vault_file.display());

    Ok(())
}

pub fn get_directory_name() -> Result<String> {
    let current_dir = env::current_dir()
        .context("Failed to get current directory")?;

    let dir_name = current_dir
        .file_name()
        .context("Failed to get directory name")?
        .to_string_lossy()
        .to_string();

    Ok(dir_name)
}
