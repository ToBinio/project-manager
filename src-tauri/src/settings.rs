use crate::error::Result;
use applications::App;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub apps: Vec<App>,
}

pub fn get_settings() -> Result<Settings> {
    let config_path = get_config_file_path()?;
    if config_path.exists().not() {
        return Ok(Settings::default());
    }

    let file_content = fs::read_to_string(config_path)?;
    Ok(serde_json::from_str(&file_content)?)
}

pub fn save_settings(settings: Settings) -> Result<()> {
    let config_path = get_config_file_path()?;

    fs::create_dir_all(config_path.parent().ok_or("No Parent found")?)?;
    fs::write(config_path, serde_json::to_string(&settings)?)?;

    Ok(())
}

fn get_config_file_path() -> Result<PathBuf> {
    Ok(get_config_dir_path()?.join("settings.json"))
}

pub fn get_config_dir_path() -> Result<PathBuf> {
    let dirs =
        ProjectDirs::from("at", "tobinio", "Project Manager").ok_or("Invalid Project Dirs")?;

    Ok(dirs.config_dir().to_path_buf())
}
