use crate::storage::Storage;
use crate::types::{ProjectConfig, SecretStore};
use crate::project;
use anyhow::Result;

pub fn execute(storage: &Storage, global: bool, project_override: Option<String>, export: bool) -> Result<()> {
    let mut final_store = SecretStore::new();
    if global {
        final_store = storage.load_global_secrets()?;
    } else {
        let global_secrets = storage.load_global_secrets()?;
        final_store.merge(&global_secrets);

        let project_config = match project_override {
            Some(project_name) => {
                Some(ProjectConfig::new(project_name))
            }
            None => {
                project::get_current_project()?
            }
        };

        if let Some(config) = project_config {
            let project_secrets = storage.load_project_secrets(&config)?;
            final_store.merge(&project_secrets);
        } else if !export {
            eprintln!("Warning: No project configured. Only global secrets loaded.");
            eprintln!("Run 'vault local [project]' to set up project secrets.");
        }
    }

    if final_store.secrets.is_empty() {
        return Ok(());
    }

    let mut keys: Vec<_> = final_store.secrets.keys().collect();
    keys.sort();

    for key in keys {
        if let Some(value) = final_store.secrets.get(key) {
            if export {
                println!("export {}='{}'", key, escape_shell_value(value));
            } else {
                println!("{}={}", key, value);
            }
        }
    }

    Ok(())
}

fn escape_shell_value(value: &str) -> String {
    value.replace('\'', r#"'"'"'"#)
}
