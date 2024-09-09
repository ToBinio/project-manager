use tauri::{Builder, Wry};

mod apps;
mod fs;
mod settings;

pub fn add_commands(app: Builder<Wry>) -> Builder<Wry> {
    app.invoke_handler(tauri::generate_handler![
        settings::get_settings,
        settings::save_settings,
        fs::get_projects,
        apps::get_all_apps,
    ])
}
