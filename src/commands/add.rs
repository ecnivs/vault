use crate::storage::Storage;
use crate::types::SecretScope;
use anyhow::{bail, Context, Result};

pub fn execute(storage: &Storage, scope: &SecretScope, secret_input: &str) -> Result<()> {
    let (key, value) = parse_secret_input(secret_input)?;

    let mut store = storage.load_secrets(scope)?;

    let is_update = store.get_secret(&key).is_some();
    store.add_secret(key.clone(), value);

    storage.save_secrets(scope, &store)?;

    if is_update {
        println!("Updated secret '{}'", key);
    } else {
        println!("Added secret '{}'", key);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_secret_input() {
        assert_eq!(
            parse_secret_input("API_KEY=secret123").unwrap(),
            ("API_KEY".to_string(), "secret123".to_string())
        );

        assert_eq!(
            parse_secret_input("DB_PASS=").unwrap(),
            ("DB_PASS".to_string(), "".to_string())
        );

        assert!(parse_secret_input("INVALID").is_err());
        assert!(parse_secret_input("=value").is_err());
    }
}
