// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use funds_platform::controller::{fund_controller};


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fund_controller::query_fund_info,fund_controller::fund_calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
