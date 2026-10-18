#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate gpui_kit as gpui;

mod app;
mod desktop;
mod ui;
mod updater;

fn main() {
    if let Err(error) = app::run() {
        eprintln!("Techdoro: {error:#}");
        if std::env::args_os().len() == 1 {
            let _ = notify_rust::Notification::new()
                .appname("Techdoro")
                .summary("Could not start Techdoro")
                .body(&format!("{error:#}"))
                .show();
        }
        std::process::exit(1);
    }
}
