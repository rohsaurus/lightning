use std::path::Path;
use open;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn open_file(_path: String) {
    // check if file exists
    if Path::new(&_path).exists() {
        println!("File exists");
        match open::that(_path.clone()) {
            Ok(()) => println!("Opened '{}' successfully.", _path),
            Err(err) => eprintln!("An error occurred when opening '{}': {}", _path, err),
        }
    }
    else {
        println!("File does not exist");
    }
}

#[tauri::command]
fn full_index_handler() {
    println!("Full index handler");

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
