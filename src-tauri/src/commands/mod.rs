use tauri::{Builder, Wry};

mod apps;
mod projects;
mod settings;

pub fn add_commands(app: Builder<Wry>) -> Builder<Wry> {
    app.invoke_handler(tauri::generate_handler![
        settings::get_settings,
        settings::save_settings,
        projects::get_projects,
        projects::run_project,
        apps::get_all_apps,
    ])
}
