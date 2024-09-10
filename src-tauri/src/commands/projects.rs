use crate::projects::Project;
use crate::settings::Settings;
use applications::App;

#[tauri::command]
pub fn get_projects(settings: Settings) -> Result<Vec<Project>, String> {
    crate::projects::get_projects(settings).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn run_project(project: Project, app: App) {
    crate::projects::run_project(project, app)
}
