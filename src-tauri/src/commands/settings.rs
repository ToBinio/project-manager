use crate::settings::Settings;

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    crate::settings::get_settings().map_err(|err| err.to_string())
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), String> {
    crate::settings::save_settings(settings).map_err(|err| err.to_string())
}
