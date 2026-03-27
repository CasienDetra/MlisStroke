// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod key_listener;

use std::thread;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::Emitter;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Start the key listener in a separate thread
            thread::spawn(move || {
                key_listener::run_listener(move |combo| {
                    let _ = window.emit("key_combo", combo);
                });
            });

            // Create the tray menu
            let quit = MenuItemBuilder::with_id("exit", "Exit").build(app)?;
            let tray_menu = MenuBuilder::new(app).items(&[&quit]).build()?;

            // Create the tray icon
            let _ = TrayIconBuilder::new()
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "exit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
