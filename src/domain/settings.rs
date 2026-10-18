use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Settings {
    pub volume: u32,
    pub focus_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    pub cycles_before_long_break: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            volume: 30,
            focus_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            cycles_before_long_break: 4,
        }
    }
}

impl Settings {
    pub fn normalize(&mut self) {
        self.volume = self.volume.min(100);
        self.focus_minutes = self.focus_minutes.clamp(1, 1440);
        self.short_break_minutes = self.short_break_minutes.clamp(1, 1440);
        self.long_break_minutes = self.long_break_minutes.clamp(1, 1440);
        self.cycles_before_long_break = self.cycles_before_long_break.clamp(1, 100);
    }

    pub fn durations(&self) -> [Duration; 3] {
        [
            self.focus_minutes,
            self.short_break_minutes,
            self.long_break_minutes,
        ]
        .map(|minutes| Duration::from_secs(u64::from(minutes) * 60))
    }
}
