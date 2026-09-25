use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime, WindowEvent,
};

fn show_main_window<R: Runtime, M: Manager<R>>(manager: &M) {
    if let Some(window) = manager.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg(target_os = "linux")]
fn configure_linux_graphics_backend() {
    // Disable WebKitGTK's accelerated compositing while investigating
    // rendering flicker on Linux graphics stacks.
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    let has_wayland = std::env::var_os("WAYLAND_DISPLAY").is_some();
    let has_xwayland = std::env::var_os("DISPLAY").is_some();
    let has_nvidia = std::path::Path::new("/proc/driver/nvidia/version").exists();

    if has_nvidia && has_wayland {
        // Avoid WebKitGTK DMA-BUF initialization failures on NVIDIA/Wayland.
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        // Keep hardware acceleration while avoiding NVIDIA explicit-sync issues.
        std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
    }

    // Prefer XWayland when both backends are available. This avoids GTK
    // protocol errors that can occur with tray icons and custom windows.
    if has_wayland && has_xwayland {
        std::env::set_var("GDK_BACKEND", "x11");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    configure_linux_graphics_backend();

    tauri::Builder::default()
        // This plugin must be registered before the other plugins.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &exit])?;

            TrayIconBuilder::new()
                .icon(
                    app.default_window_icon()
                        .cloned()
                        .expect("window icon is configured"),
                )
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "exit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Techdoro");
}
