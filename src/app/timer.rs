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
        self.tick(cx);
        self.timer.toggle(Instant::now());
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
        if let Some(finished) = self.timer.tick(&self.saved.settings, now) {
            self.audio.play(Sound::Alarm, self.saved.settings.volume);
            crate::desktop::notify_finished(finished == Session::Focus);
        }
        let seconds = self.timer.remaining(now).as_secs();
        if self.last_seconds != seconds {
            self.last_seconds = seconds;
            cx.notify();
        }
        self.receive_updates(cx);
    }
}
