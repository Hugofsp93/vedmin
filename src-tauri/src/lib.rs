use tauri::tray::TrayIconBuilder;
use tauri::Manager;

mod video_processor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let tray_image = tauri::image::Image::from_bytes(include_bytes!("../icons/menu-bar-icon.png")).unwrap();
            let tray_icon = TrayIconBuilder::with_id("main-tray")
                .icon(tray_image)
                .on_tray_icon_event(move |_, event| match event {
                    tauri::tray::TrayIconEvent::Click { .. } => {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = app_handle.set_activation_policy(tauri::ActivationPolicy::Regular);
                        }
                        if let Some(tray) = app_handle.tray_by_id("main-tray") {
                            let _ = tray.set_visible(false);
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            let _ = tray_icon.set_visible(false);

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                let app = window.app_handle();
                let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                if let Some(tray) = app.tray_by_id("main-tray") {
                    let _ = tray.set_visible(true);
                }
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            video_processor::process_video_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
