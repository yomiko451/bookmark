// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serve;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            serve::init,
            serve::add_bookmark,
            serve::update_id,
            serve::modify_bookmark,
            serve::delete_bookmark
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
