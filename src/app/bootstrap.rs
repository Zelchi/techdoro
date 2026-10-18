use super::{
    CheckUpdates, Hide, Model, Next, Quit, Reset, Runtime, Show, Toggle,
    cli::Startup,
    tray::{build_tray, refresh_tray},
    window::{hide_main, show_main, track_window_close},
};
use anyhow::Result;
use gpui_kit::*;
use std::{rc::Rc, time::Duration};

pub(super) fn start(startup: Startup) -> Result<()> {
    // Package upgrades replace the running executable on Linux. Capture its path
    // before that happens so restart never resolves a path ending in " (deleted)".
    let restart_path = std::env::current_exe()?;
    let Startup {
        saved,
        storage,
        instance,
    } = startup;
    let instance = Rc::new(instance);
    application().with_assets(assets::Assets).run(move |cx| {
        gpui_kit::init(cx);
        cx.set_restart_path(restart_path);
        cx.set_app_identity("com.zelchi.techdoro", "Techdoro");
        cx.set_quit_mode(QuitMode::Explicit);
        crate::ui::init_theme(cx);
        let model = cx.new(|_| Model::new(saved, storage));
        cx.set_global(Runtime {
            model: model.clone(),
            window: None,
            window_opening: false,
            tray: None,
        });
        track_window_close(cx);
        cx.on_action(|_: &Show, cx| cx.defer(show_main));
        cx.on_action(|_: &Hide, cx| cx.defer(hide_main));
        cx.on_action(|_: &Toggle, cx| {
            let model = cx.global::<Runtime>().model.clone();
            model.update(cx, |model, cx| model.toggle(cx));
        });
        cx.on_action(|_: &Reset, cx| {
            let model = cx.global::<Runtime>().model.clone();
            model.update(cx, |model, cx| model.reset(cx));
        });
        cx.on_action(|_: &Next, cx| {
            let model = cx.global::<Runtime>().model.clone();
            model.update(cx, |model, cx| model.next(cx));
        });
        cx.on_action(|_: &CheckUpdates, cx| {
            let model = cx.global::<Runtime>().model.clone();
            model.update(cx, |model, cx| model.check_updates(cx));
        });
        cx.on_action(|_: &Quit, cx| cx.quit());
        match build_tray(cx) {
            Ok(tray) => cx.global_mut::<Runtime>().tray = Some(tray),
            Err(error) => eprintln!("System tray unavailable: {error:#}"),
        }
        show_main(cx);
        model.update(cx, |model, cx| model.check_updates(cx));
        cx.spawn(async move |cx| {
            let mut was_running = false;
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
                cx.update(|cx| {
                    model.update(cx, |model, cx| model.tick(cx));
                    let running = model.read(cx).timer.running();
                    if running != was_running {
                        was_running = running;
                        refresh_tray(cx);
                    }
                    if instance.take_activation() {
                        show_main(cx);
                    }
                });
            }
        })
        .detach();
    });
    Ok(())
}
