use super::{CheckUpdates, Hide, Next, Quit, Reset, Runtime, Show, Toggle};
use anyhow::Result;
use gpui_kit::*;

pub(super) fn build_tray(cx: &mut App) -> Result<gpui_tray::Tray> {
    let image = image::load_from_memory(include_bytes!("../../assets/icon.png"))?
        .resize_exact(32, 32, image::imageops::FilterType::Lanczos3)
        .into_rgba8();
    let icon = gpui_tray::Icon::from_rgba(image.as_raw().clone(), image.width(), image.height())?;
    Ok(gpui_tray::Tray::builder()
        .icon(icon)
        .title("Techdoro")
        .on_activate(Show)
        .menu(|cx| {
            let runtime = cx.global::<Runtime>();
            let running = runtime.model.read(cx).timer.running();
            let visibility = if runtime.window.is_some() {
                MenuItem::action("Hide", Hide)
            } else {
                MenuItem::action("Show", Show)
            };
            vec![
                visibility,
                MenuItem::separator(),
                MenuItem::action(if running { "Pause" } else { "Start" }, Toggle),
                MenuItem::action("Reset", Reset),
                MenuItem::action("Next", Next),
                MenuItem::separator(),
                MenuItem::action("Update", CheckUpdates),
                MenuItem::action("Exit", Quit),
            ]
        })
        .build(cx)?)
}

pub(super) fn refresh_tray(cx: &mut App) {
    if let Some(tray) = cx.global::<Runtime>().tray.clone()
        && let Err(error) = tray.refresh_menu(cx)
    {
        eprintln!("System tray: {error}");
    }
}
