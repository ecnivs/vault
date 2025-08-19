use crate::storage::Storage;
use crate::types::SecretScope;
use anyhow::Result;

pub fn execute(storage: &Storage, scope: &SecretScope) -> Result<()> {
    let store = storage.load_secrets(scope)?;

    if store.secrets.is_empty() {
        match (&scope.project, &scope.environment) {
            (None, None) => println!("No global secrets found"),
            (Some(proj), Some(env)) => println!("No secrets found for {} {}", proj, env),
            _ => unreachable!(),
        }
        return Ok(());
    }

    match (&scope.project, &scope.environment) {
        (None, None) => println!("Global secrets:"),
        (Some(proj), Some(env)) => println!("Secrets for {} {}:", proj, env),
        _ => unreachable!(),
    }

    let mut keys: Vec<_> = store.secrets.keys().collect();
    keys.sort();

    for key in keys {
        println!("  {}", key);
    }

    println!("\n{} secret(s) total", store.secrets.len());

    Ok(())
}
