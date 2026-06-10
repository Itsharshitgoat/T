pub mod ai;
pub mod cursor;
pub mod memory;
pub mod window;

use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            
            // Set activation policy to Accessory to hide from Dock
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_dir = app.path().app_data_dir().unwrap();
            std::fs::create_dir_all(&app_dir).unwrap();
            
            let db = memory::init_db(&app_dir).unwrap();
            app.manage(memory::DbState {
                db: Mutex::new(db),
            });

            cursor::start_tracking(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ai::send_prompt,
            memory::add_conversation,
            memory::get_recent_conversations,
            memory::get_preference,
            memory::set_preference,
            window::show_window,
            window::hide_window,
            window::set_window_position
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

