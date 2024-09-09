use applications::App;

#[tauri::command]
pub fn get_all_apps() -> Result<Vec<App>, String> {
    crate::apps::get_all_apps().map_err(|err| err.to_string())
}
