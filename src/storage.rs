use crate::types::{SecretScope, SecretStore};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Storage {
    base_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let base_dir = dirs::state_dir()
            .context("Could not find state directory")?
            .join("vault");

        fs::create_dir_all(&base_dir)
            .context("Failed to create vault directory")?;

        Ok(Self { base_dir })
    }
 
    pub fn load_secrets(&self, scope: &SecretScope) -> Result<SecretStore> {
        let path = self.base_dir.join(&scope.filename());

        if !path.exists() {
            return Ok(SecretStore::new());
        }

        let content = fs::read_to_string(&path)
            .context("Failed to read secrets file")?;

        let store: SecretStore = serde_yaml::from_str(&content)
            .context("Failed to parse secrets file")?;

        Ok(store)
    }

    pub fn save_secrets(&self, scope: &SecretScope, store: &SecretStore) -> Result<()> {
        let path = self.base_dir.join(&scope.filename());

        let content = serde_yaml::to_string(store)
            .context("Failed to serialize secrets")?;

        fs::write(&path, content)
            .context("Failed to write secrets file")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600); // rw
            fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    pub fn list_scopes(&self) -> Result<Vec<SecretScope>> {
        let mut scopes = vec![SecretScope::global()];

        let projects_dir = self.base_dir.join("projects");
        if projects_dir.exists() {
            for entry in fs::read_dir(projects_dir)? {
                let entry = entry?;
                let filename = entry.file_name().to_string_lossy().to_string();

                if filename.ends_with(".yml") {
                    let name = filename.trim_end_matches(".yml");
                    if let Some((project, env)) = name.split_once('.') {
                        scopes.push(SecretScope::project(
                            project.to_string(),
                            env.to_string(),
                        ));
                    }
                }
            }
        }

        Ok(scopes)
    }
}
