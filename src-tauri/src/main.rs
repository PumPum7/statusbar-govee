// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod fns;
mod tray;

use tauri::{Builder, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;

fn main() {
    Builder::default()
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            command::init,
            command::show_menubar_panel,
            command::get_devices,
            command::get_device_state,
            command::change_capability_value,
            command::get_api_key,
            command::set_api_key,
        ])
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.app_handle();

            tray::create(app_handle)?;

            let autostart_manager = app.autolaunch();
            let _ = autostart_manager.enable();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
