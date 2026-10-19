use super::Model;
use crate::desktop::Sound;
use gpui_kit::*;
use std::time::Instant;
use techdoro::domain::Session;

impl Model {
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.installing_update {
            return;
        }
        let now = Instant::now();
        let finished = self.timer.toggle_unless_finished(&self.saved.settings, now);
        self.update_after_tick(now, finished, cx);
        self.click();
        cx.notify();
    }

    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.timer.reset(&self.saved.settings);
        self.click();
        cx.notify();
    }

    pub fn next(&mut self, cx: &mut Context<Self>) {
        self.timer.next(Instant::now());
        self.click();
        cx.notify();
    }

    pub fn settings_changed(&mut self, cx: &mut Context<Self>) {
        self.saved.settings.normalize();
        self.audio.set_volume(self.saved.settings.volume);
        self.timer.configure(&self.saved.settings, Instant::now());
        self.persist();
        self.click();
        cx.notify();
    }

    pub fn tick(&mut self, cx: &mut Context<Self>) {
        let now = Instant::now();
        let finished = self.timer.tick(&self.saved.settings, now);
        self.update_after_tick(now, finished, cx);
    }

    fn update_after_tick(
        &mut self,
        now: Instant,
        finished: Option<Session>,
        cx: &mut Context<Self>,
    ) {
        if let Some(finished) = finished {
            self.audio.play(Sound::Alarm, self.saved.settings.volume);
            crate::desktop::notify_finished(finished == Session::Focus);
            cx.defer(super::window::show_main);
        }
        let seconds = self.timer.remaining(now).as_secs();
        if self.last_seconds != seconds {
            self.last_seconds = seconds;
            cx.notify();
        }
        self.receive_updates(cx);
    }
}
