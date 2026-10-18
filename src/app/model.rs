use crate::{
    desktop::{Audio, Sound},
    updater::{self, UpdateMessage},
};
use std::sync::mpsc;
use techdoro::{
    domain::{SavedState, Timer},
    storage::Storage,
};

pub struct Model {
    pub saved: SavedState,
    pub timer: Timer,
    pub error: Option<String>,
    pub update_status: String,
    pub update_busy: bool,
    pub installing_update: bool,
    pub update: Option<updater::AvailableUpdate>,
    pub(super) storage: Storage,
    pub(super) audio: Audio,
    pub(super) sender: mpsc::Sender<UpdateMessage>,
    pub(super) receiver: mpsc::Receiver<UpdateMessage>,
    pub(super) last_seconds: u64,
}

impl Model {
    pub fn new(saved: SavedState, storage: Storage) -> Self {
        let timer = Timer::new(&saved.settings);
        let (sender, receiver) = mpsc::channel();
        Self {
            saved,
            timer,
            storage,
            audio: Audio::new(),
            sender,
            receiver,
            error: None,
            update_status: String::new(),
            update_busy: false,
            installing_update: false,
            update: None,
            last_seconds: u64::MAX,
        }
    }

    pub fn persist(&mut self) {
        self.error = self
            .storage
            .save(&self.saved)
            .err()
            .map(|error| format!("Could not save: {error}"));
    }

    pub fn click(&self) {
        self.audio.play(Sound::Click, self.saved.settings.volume);
    }
}
