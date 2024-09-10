mod apps;
mod commands;
mod error;
mod projects;
mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init());

    let app = commands::add_commands(app);

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
