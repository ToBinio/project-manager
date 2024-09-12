use applications::App;
use tracing::error;
#[tauri::command]
pub fn get_all_apps() -> Result<Vec<App>, String> {
    crate::apps::get_all_apps().map_err(|err| {
        error!("{}", err.to_string());
        err.to_string()
    })
}
