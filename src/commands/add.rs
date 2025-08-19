use crate::storage::Storage;
use crate::types::ProjectConfig;
use crate::project;
use anyhow::{bail, Result};

pub fn execute(storage: &Storage, global: bool, project_override: Option<String>, secret_input: &str) -> Result<()> {
    let (key, value) = parse_secret_input(secret_input)?;
    if global {
        let mut store = storage.load_global_secrets()?;
        let is_update = store.get_secret(&key).is_some();
        store.add_secret(key.clone(), value);
        storage.save_global_secrets(&store)?;

        if is_update {
            println!("Updated global secret '{}'", key);
        } else {
            println!("Added global secret '{}'", key);
        }
    } else {
        let project_config = match project_override {
            Some(project_name) => {
                ProjectConfig::new(project_name)
            }
            None => {
                project::get_current_project()?
                    .ok_or_else(|| anyhow::anyhow!(
                        "No project configured. Run 'vault local [project]' first, use --project <name>, or use --global"
                    ))?
            }
        };

        let mut store = storage.load_project_secrets(&project_config)?;
        let is_update = store.get_secret(&key).is_some();
        store.add_secret(key.clone(), value);
        storage.save_project_secrets(&project_config, &store)?;

        if is_update {
            println!("Updated secret '{}' for {}", key, project_config.display());
        } else {
            println!("Added secret '{}' for {}", key, project_config.display());
        }
    }

    Ok(())
}

fn parse_secret_input(input: &str) -> Result<(String, String)> {
    if let Some((key, value)) = input.split_once('=') {
        if key.is_empty() {
            bail!("Secret key cannot be empty");
        }
        Ok((key.to_string(), value.to_string()))
    } else {
        bail!("Secret must be in KEY=VALUE format");
    }
}
