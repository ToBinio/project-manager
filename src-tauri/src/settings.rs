use derive_more::From;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;
#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    NoParent,
    InvalidProjectDir,

    #[from]
    IO(#[serde_as(as = "DisplayFromStr")] std::io::Error),
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

#[derive(Deserialize, Serialize, Default)]
pub struct Settings {
    path: Option<PathBuf>,
}

#[tauri::command]
pub fn get_settings() -> Result<Settings> {
    let config_path = get_config_path()?;
    if config_path.exists().not() {
        return Ok(Settings::default());
    }

    let file_content = fs::read_to_string(config_path)?;
    Ok(serde_json::from_str(&file_content)?)
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<()> {
    let config_path = get_config_path()?;

    fs::create_dir_all(&config_path.parent().ok_or_else(|| Error::NoParent)?)?;
    fs::write(config_path, serde_json::to_string(&settings)?)?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("at", "tobinio", "Project Manager")
        .ok_or_else(|| Error::InvalidProjectDir)?;

    Ok(dirs.config_dir().to_path_buf().join("settings.json"))
}
