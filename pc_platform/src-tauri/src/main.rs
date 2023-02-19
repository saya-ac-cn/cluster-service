#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use pc_platform::controller::{os_controller};

fn window_event(e:tauri::GlobalWindowEvent){
    match e.event(){
        tauri::WindowEvent::Focused(focused) => {
            if !focused{
                e.window().hide().unwrap();
            }
        }
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
        //.on_window_event(window_event)
        .invoke_handler(tauri::generate_handler![os_controller::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}