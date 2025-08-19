use crate::storage::Storage;
use crate::types::SecretScope;
use anyhow::Result;

pub fn execute(storage: &Storage, scope: &SecretScope, export: bool) -> Result<()> {
    let store = storage.load_secrets(scope)?;

    if store.secrets.is_empty() {
        return Ok(());
    }
    let mut keys: Vec<_> = store.secrets.keys().collect();
    keys.sort();

    for key in keys {
        if let Some(value) = store.secrets.get(key) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_shell_value() {
        assert_eq!(escape_shell_value("simple"), "simple");
        assert_eq!(escape_shell_value("with spaces"), "with spaces");
        assert_eq!(escape_shell_value("with'quote"), r#"with'"'"'quote"#);
        assert_eq!(escape_shell_value("multiple'quotes'here"), r#"multiple'"'"'quotes'"'"'here"#);
    }
}
