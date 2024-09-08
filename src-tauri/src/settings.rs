use crate::error::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Settings {
    pub path: Option<PathBuf>,
}

pub fn get_settings() -> Result<Settings> {
    let config_path = get_config_path()?;
    if config_path.exists().not() {
        return Ok(Settings::default());
    }

    let file_content = fs::read_to_string(config_path)?;
    Ok(serde_json::from_str(&file_content)?)
}

pub fn save_settings(settings: Settings) -> Result<()> {
    let config_path = get_config_path()?;

    fs::create_dir_all(&config_path.parent().ok_or_else(|| "No Parent found")?)?;
    fs::write(config_path, serde_json::to_string(&settings)?)?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("at", "tobinio", "Project Manager")
        .ok_or_else(|| "Invalid Project Dirs")?;

    Ok(dirs.config_dir().to_path_buf().join("settings.json"))
}
