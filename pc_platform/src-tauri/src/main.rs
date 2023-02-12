#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use pc_platform::controller::{os_controller};


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![os_controller::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
