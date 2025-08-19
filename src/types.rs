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

    pub fn merge(&mut self, other: &SecretStore) {
        for (key, value) in &other.secrets {
            self.secrets.insert(key.clone(), value.clone());
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectConfig {
    pub name: String,
}

impl ProjectConfig {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn filename(&self) -> String {
        format!("{}.yml", self.name)
    }

    pub fn display(&self) -> String {
        self.name.clone()
    }
}
