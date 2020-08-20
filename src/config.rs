use serde_derive::*;
use anyhow::Error;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    SetCurrentScene {
        button: usize,
        scene: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub commands: Vec<Command>
}

impl Configuration {
    pub fn load_from(path: &Path) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path)?;
        let deserialized = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }
}
