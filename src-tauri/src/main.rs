// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod key_listener;

use std::thread;
use tauri::Manager;
use tauri::Emitter;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
let window = app.get_webview_window("main").unwrap();

            thread::spawn(move || {
                key_listener::run_listener(move |combo| {
                    let _ = window.emit("key_combo", combo);
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
