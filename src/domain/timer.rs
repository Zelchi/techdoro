use super::Settings;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Session {
    Focus,
    ShortBreak,
    LongBreak,
}

impl Session {
    pub fn index(self) -> usize {
        match self {
            Self::Focus => 0,
            Self::ShortBreak => 1,
            Self::LongBreak => 2,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Focus => "Focus",
            Self::ShortBreak => "Short Break",
            Self::LongBreak => "Long Break",
        }
    }
}

pub struct Timer {
    pub session: Session,
    pub completed_focus_cycles: u32,
    remaining: [Duration; 3],
    maximum: [Duration; 3],
    started_at: Option<Instant>,
}

impl Timer {
    pub fn new(settings: &Settings) -> Self {
        let durations = settings.durations();
        Self {
            session: Session::Focus,
            completed_focus_cycles: 0,
            remaining: durations,
            maximum: durations,
            started_at: None,
        }
    }

    pub fn running(&self) -> bool {
        self.started_at.is_some()
    }

    pub fn remaining(&self, now: Instant) -> Duration {
        let elapsed = self
            .started_at
            .map(|start| now.saturating_duration_since(start))
            .unwrap_or_default();
        self.remaining[self.session.index()].saturating_sub(elapsed)
    }

    pub fn maximum(&self) -> Duration {
        self.maximum[self.session.index()]
    }

    pub fn toggle(&mut self, now: Instant) {
        if self.running() {
            self.pause(now);
        } else {
            self.started_at = Some(now);
        }
    }

    fn pause(&mut self, now: Instant) {
        self.remaining[self.session.index()] = self.remaining(now);
        self.started_at = None;
    }

    pub fn reset(&mut self, settings: &Settings) {
        self.started_at = None;
        let index = self.session.index();
        self.maximum[index] = settings.durations()[index];
        self.remaining[index] = self.maximum[index];
    }

    pub fn next(&mut self, now: Instant) {
        self.pause(now);
        self.session = match self.session {
            Session::Focus => Session::ShortBreak,
            Session::ShortBreak => Session::LongBreak,
            Session::LongBreak => Session::Focus,
        };
    }

    pub fn configure(&mut self, settings: &Settings, now: Instant) {
        let was_running = self.running();
        self.pause(now);
        self.maximum = settings.durations();
        for (remaining, maximum) in self.remaining.iter_mut().zip(self.maximum) {
            *remaining = (*remaining).min(maximum);
        }
        if was_running {
            self.started_at = Some(now);
        }
    }

    pub fn tick(&mut self, settings: &Settings, now: Instant) -> Option<Session> {
        if !self.running() || !self.remaining(now).is_zero() {
            return None;
        }
        let finished = self.session;
        self.remaining[finished.index()] = self.maximum[finished.index()];
        self.started_at = None;
        self.session = if finished == Session::Focus {
            self.completed_focus_cycles += 1;
            if self.completed_focus_cycles >= settings.cycles_before_long_break {
                self.completed_focus_cycles = 0;
                Session::LongBreak
            } else {
                Session::ShortBreak
            }
        } else {
            Session::Focus
        };
        self.reset(settings);
        Some(finished)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pause_and_resume_preserve_elapsed_time() {
        let mut timer = Timer::new(&Settings::default());
        let now = Instant::now();
        timer.toggle(now);
        timer.toggle(now + Duration::from_secs(10));
        assert_eq!(
            timer.remaining(now + Duration::from_secs(100)),
            Duration::from_secs(1490)
        );
        timer.toggle(now + Duration::from_secs(100));
        assert_eq!(
            timer.remaining(now + Duration::from_secs(105)),
            Duration::from_secs(1485)
        );
    }

    #[test]
    fn long_break_follows_four_completed_focus_sessions() {
        let settings = Settings::default();
        let mut timer = Timer::new(&settings);
        let now = Instant::now();
        for cycle in 1..=4 {
            assert_eq!(timer.session, Session::Focus);
            timer.toggle(now);
            assert_eq!(
                timer.tick(&settings, now + Duration::from_secs(1500)),
                Some(Session::Focus)
            );
            assert!(!timer.running());
            assert_eq!(timer.tick(&settings, now + Duration::from_secs(9000)), None);
            assert_eq!(
                timer.session,
                if cycle == 4 {
                    Session::LongBreak
                } else {
                    Session::ShortBreak
                }
            );
            timer.toggle(now);
            timer.tick(&settings, now + Duration::from_secs(900));
        }
        assert_eq!(timer.session, Session::Focus);
        assert_eq!(timer.completed_focus_cycles, 0);
    }

    #[test]
    fn manual_skip_preserves_clocks_without_counting_cycles() {
        let mut timer = Timer::new(&Settings::default());
        let now = Instant::now();
        timer.toggle(now);
        timer.next(now + Duration::from_secs(20));
        timer.next(now);
        timer.next(now);
        assert_eq!(timer.remaining(now), Duration::from_secs(1480));
        assert_eq!(timer.completed_focus_cycles, 0);
        assert!(!timer.running());
    }

    #[test]
    fn changing_settings_accounts_for_elapsed_and_clamps_remaining() {
        let mut settings = Settings::default();
        let mut timer = Timer::new(&settings);
        let now = Instant::now();
        timer.toggle(now);
        settings.focus_minutes = 1;
        timer.configure(&settings, now + Duration::from_secs(10));
        assert_eq!(
            timer.remaining(now + Duration::from_secs(15)),
            Duration::from_secs(55)
        );
        assert!(timer.running());
    }
}
