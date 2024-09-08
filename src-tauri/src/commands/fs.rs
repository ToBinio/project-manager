use crate::settings::Settings;

#[tauri::command]
pub fn get_projects(settings: Settings) -> Result<Vec<String>, String> {
    crate::fs::get_projects(settings).map_err(|err| err.to_string())
}
