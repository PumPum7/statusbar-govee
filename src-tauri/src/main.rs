// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod fns;
mod tray;

use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .invoke_handler(tauri::generate_handler![
            command::init,
            command::show_menubar_panel,
            command::get_devices,
            command::get_device_state,
            command::change_capability_value
        ])
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.app_handle();

            tray::create(app_handle)?;

            // Get the autostart manager
            let autostart_manager = app.autolaunch();
            // Enable autostart
            let _ = autostart_manager.enable();
            // Check enable state
            println!(
                "registered for autostart? {}",
                autostart_manager.is_enabled().unwrap()
            );
            // Disable autostart
            let _ = autostart_manager.disable();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
