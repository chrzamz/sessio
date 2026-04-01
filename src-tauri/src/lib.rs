mod commands;
mod db;
mod parser;
mod scanner;

use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_data_dir = dirs::data_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap().join(".local/share"))
        .join("sessio");
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

    let db_path = app_data_dir.join("index.db");
    let database = Database::new(&db_path).expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            commands::scan_and_index,
            commands::list_sessions,
            commands::get_session_messages,
            commands::search,
            commands::toggle_star,
            commands::get_stats,
            commands::get_projects,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
