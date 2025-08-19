use crate::storage::Storage;
use crate::types::ProjectConfig;
use crate::project;
use anyhow::{bail, Result};

pub fn execute(storage: &Storage, global: bool, project_override: Option<String>, key: &str) -> Result<()> {
    if global {
        let mut store = storage.load_global_secrets()?;
        match store.remove_secret(key) {
            Some(_) => {
                storage.save_global_secrets(&store)?;
                println!("Removed global secret '{}'", key);
            }
            None => {
                bail!("Global secret '{}' not found", key);
            }
        }
    } else {
        let project_config = match project_override {
            Some(project_name) => {
                ProjectConfig::new(project_name)
            }
            None => {
                project::get_current_project()?
                    .ok_or_else(|| anyhow::anyhow!(
                        "No project configured. Run 'vault local [project]' first, use --project <n>, or use --global"
                    ))?
            }
        };

        let mut store = storage.load_project_secrets(&project_config)?;
        match store.remove_secret(key) {
            Some(_) => {
                storage.save_project_secrets(&project_config, &store)?;
                println!("Removed secret '{}' from {}", key, project_config.display());
            }
            None => {
                bail!("Secret '{}' not found for {}", key, project_config.display());
            }
        }
    }
    Ok(())
}
