use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretStore {
    pub secrets: HashMap<String, String>,
}

impl SecretStore {
    pub fn new() -> Self {
        Self {
            secrets: HashMap::new(),
        }
    }

    pub fn add_secret(&mut self, key: String, value: String) {
        self.secrets.insert(key, value);
    }

    pub fn remove_secret(&mut self, key: &str) -> Option<String> {
        self.secrets.remove(key)
    }

    pub fn get_secret(&self, key: &str) -> Option<&String> {
        self.secrets.get(key)
    }
}

#[derive(Debug, Clone)]
pub struct SecretScope {
    pub project: Option<String>,
    pub environment: Option<String>,
}

impl SecretScope {
    pub fn global() -> Self {
        Self {
            project: None,
            environment: None,
        }
    }

    pub fn project(project: String, env: String) -> Self {
        Self {
            project: Some(project),
            environment: Some(env),
        }
    }

    pub fn filename(&self) -> String {
        match (&self.project, &self.environment) {
            (None, None) => "global.yml".to_string(),
            (Some(proj), Some(env)) => format!("{}.{}.yml", proj, env),
            _ => panic!("Invalid scope: project and environment must both be Some or None"),
        }
    }
}
