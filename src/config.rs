use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub program_paths: Vec<String>,

    #[serde(default)]
    pub project_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            program_paths: vec![],
            project_paths: vec![],
        }
    }
}
