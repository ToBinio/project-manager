mod commands;
mod error;
mod settings;
mod fs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init());

    let app = commands::add_commands(app);

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
