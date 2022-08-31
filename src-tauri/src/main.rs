#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn exit_program() {
    panic!("");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exit_program])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
