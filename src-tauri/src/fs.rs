use crate::error;
use crate::settings::Settings;
use std::fs::read_dir;

pub fn get_projects(settings: Settings) -> error::Result<Vec<String>> {
    let names = read_dir(settings.path.unwrap_or_default())?
        .filter_map(Result::ok)
        .filter_map(|dir| dir.file_name().into_string().ok())
        .collect();

    Ok(names)
}
