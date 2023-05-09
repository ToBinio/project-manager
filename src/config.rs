use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub program_paths: Vec<String>,

    #[serde(default)]
    pub project_paths: Vec<String>,
}
