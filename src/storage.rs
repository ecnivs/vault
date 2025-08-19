use crate::types::{ProjectConfig, SecretStore};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

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

    pub fn load_global_secrets(&self) -> Result<SecretStore> {
        let path = self.base_dir.join("global.yml");
        self.load_secrets_from_file(&path)
    }

    pub fn save_global_secrets(&self, store: &SecretStore) -> Result<()> {
        let path = self.base_dir.join("global.yml");
        self.save_secrets_to_file(&path, store)
    }

    pub fn load_project_secrets(&self, config: &ProjectConfig) -> Result<SecretStore> {
        let path = self.base_dir.join(&config.filename());
        self.load_secrets_from_file(&path)
    }

    pub fn save_project_secrets(&self, config: &ProjectConfig, store: &SecretStore) -> Result<()> {
        let path = self.base_dir.join(&config.filename());
        self.save_secrets_to_file(&path, store)
    }

    fn load_secrets_from_file(&self, path: &PathBuf) -> Result<SecretStore> {
        if !path.exists() {
            return Ok(SecretStore::new());
        }

        let content = fs::read_to_string(path)
            .context("Failed to read secrets file")?;

        let store: SecretStore = serde_yaml::from_str(&content)
            .context("Failed to parse secrets file")?;

        Ok(store)
    }

    fn save_secrets_to_file(&self, path: &PathBuf, store: &SecretStore) -> Result<()> {
        let content = serde_yaml::to_string(store)
            .context("Failed to serialize secrets")?;

        fs::write(path, content)
            .context("Failed to write secrets file")?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(0o600); // rw
            fs::set_permissions(path, perms)?;
        }

        Ok(())
    }
}
