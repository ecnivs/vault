use crate::storage::Storage;
use crate::types::ProjectConfig;
use crate::project;
use anyhow::Result;

pub fn execute(storage: &Storage, global: bool, project_override: Option<String>) -> Result<()> {
    if global {
        let store = storage.load_global_secrets()?;
        if store.secrets.is_empty() {
            println!("No global secrets found");
            return Ok(());
        }
        println!("Global secrets:");
        let mut keys: Vec<_> = store.secrets.keys().collect();
        keys.sort();
        for key in keys {
            println!("  {}", key);
        }
        println!("\n{} global secret(s)", store.secrets.len());
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

        let store = storage.load_project_secrets(&project_config)?;

        if store.secrets.is_empty() {
            println!("No secrets found for {}", project_config.display());
            return Ok(());
        }

        println!("Secrets for {}:", project_config.display());
        let mut keys: Vec<_> = store.secrets.keys().collect();
        keys.sort();

        for key in keys {
            println!("  {}", key);
        }

        println!("\n{} secret(s)", store.secrets.len());
    }

    Ok(())
}
