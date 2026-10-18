use super::{Runtime, tray::refresh_tray};
use gpui_kit::*;
use std::sync::Arc;

pub(super) fn show_main(cx: &mut App) {
    if cx.global::<Runtime>().window_opening {
        return;
    }
    if let Some(handle) = cx.global::<Runtime>().window {
        if cx.windows().contains(&handle) {
            cx.defer(move |cx| {
                let _ = handle.update(cx, |_, window, cx| {
                    window.activate_window();
                    cx.activate(true);
                });
            });
            return;
        }
        cx.global_mut::<Runtime>().window = None;
    }
    cx.global_mut::<Runtime>().window_opening = true;
    let model = cx.global::<Runtime>().model.clone();
    let icon = image::load_from_memory(include_bytes!("../../assets/icon.png"))
        .ok()
        .map(|image| Arc::new(image.into_rgba8()));
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(500.), px(600.)),
            cx,
        ))),
        titlebar: None,
        window_decorations: Some(WindowDecorations::Client),
        app_owns_titlebar_drag: true,
        is_resizable: false,
        window_min_size: Some(size(px(500.), px(600.))),
        app_id: Some("com.zelchi.techdoro".into()),
        icon,
        ..Default::default()
    };
    let result = gpui_kit::open_window(options, cx, |window, cx| {
        window.set_window_title("Techdoro");
        window.on_window_should_close(cx, |_, cx| {
            if cx.global::<Runtime>().tray.is_none() {
                cx.quit();
            }
            true
        });
        cx.new(|cx| crate::ui::View::new(model, window, cx))
    });
    cx.global_mut::<Runtime>().window_opening = false;
    match result {
        Ok((window, _)) => {
            cx.global_mut::<Runtime>().window = Some(window);
            cx.activate(true);
            refresh_tray(cx);
        }
        Err(error) => {
            eprintln!("Could not open window: {error:#}");
            cx.quit();
        }
    }
}

pub(super) fn hide_main(cx: &mut App) {
    if cx.global::<Runtime>().tray.is_none() {
        cx.quit();
        return;
    }
    if let Some(handle) = cx.global::<Runtime>().window {
        let _ = handle.update(cx, |_, window, _| window.remove_window());
    }
}

pub(super) fn track_window_close(cx: &mut App) {
    cx.on_window_closed(|cx, id| {
        if cx
            .global::<Runtime>()
            .window
            .is_some_and(|handle| handle.window_id() == id)
        {
            cx.global_mut::<Runtime>().window = None;
            refresh_tray(cx);
        }
    })
    .detach();
}
