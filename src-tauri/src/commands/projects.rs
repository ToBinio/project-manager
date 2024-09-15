use crate::projects::{Project, ProjectMetaData};
use crate::settings::Settings;
use applications::App;
use tracing::error;

#[tauri::command]
pub fn get_projects(settings: Settings) -> Result<Vec<Project>, String> {
    crate::projects::get_projects(settings).map_err(|err| {
        error!("{}", err.to_string());
        err.to_string()
    })
}

#[tauri::command]
pub fn run_project(mut project: Project, mut app: App) {
    crate::projects::run_project(&mut project, &mut app)
}

#[tauri::command]
pub fn get_project_metadata(project: Project) -> Result<ProjectMetaData, String> {
    crate::projects::get_metadata(&project.name).map_err(|err| {
        error!("{}", err.to_string());
        err.to_string()
    })
}
