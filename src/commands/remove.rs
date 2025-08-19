use crate::storage::Storage;
use crate::types::SecretScope;
use anyhow::{bail, Result};

pub fn execute(storage: &Storage, scope: &SecretScope, key: &str) -> Result<()> {
    let mut store = storage.load_secrets(scope)?;

    match store.remove_secret(key) {
        Some(_) => {
            storage.save_secrets(scope, &store)?;
            println!("Removed secret '{}'", key);
        }
        None => {
            bail!("Secret '{}' not found", key);
        }
    }

    Ok(())
}
