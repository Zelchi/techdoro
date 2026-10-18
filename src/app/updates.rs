use super::Model;
use crate::updater::{self, UpdateMessage};
use gpui_kit::*;

impl Model {
    pub fn check_updates(&mut self, cx: &mut Context<Self>) {
        if self.update_busy {
            return;
        }
        self.update_busy = true;
        self.update_status = "Checking for updates...".into();
        updater::check(self.sender.clone());
        cx.notify();
    }

    pub fn install_update(&mut self, cx: &mut Context<Self>) {
        if self.timer.running() || self.update_busy {
            return;
        }
        self.persist();
        if self.error.is_some() {
            cx.notify();
            return;
        }
        if let Some(update) = self.update.clone() {
            self.update_busy = true;
            self.installing_update = true;
            self.update_status = "Downloading and installing update...".into();
            updater::install(update, self.sender.clone());
            cx.notify();
        }
    }

    pub(super) fn receive_updates(&mut self, cx: &mut Context<Self>) {
        while let Ok(message) = self.receiver.try_recv() {
            self.update_busy = false;
            match message {
                UpdateMessage::Available(update) => {
                    self.update_status = format!("Version {} available.", update.version());
                    self.update = Some(*update);
                }
                UpdateMessage::Current => {
                    self.update = None;
                    self.update_status.clear();
                }
                UpdateMessage::NotPackaged => {
                    self.update = None;
                    self.update_status.clear();
                }
                UpdateMessage::Status(status) => {
                    self.update_busy = true;
                    self.update_status = status.into();
                }
                UpdateMessage::CheckFailed(error) => {
                    eprintln!("Update check: {error}");
                    self.update_status = self
                        .update
                        .as_ref()
                        .map(|update| format!("Version {} available.", update.version()))
                        .unwrap_or_default();
                }
                UpdateMessage::InstallFailed(error) => {
                    self.installing_update = false;
                    eprintln!("Updater: {error}");
                    self.update_status = "Update failed. Please try again.".into();
                }
                UpdateMessage::Installed => {
                    self.persist();
                    cx.restart();
                }
            }
            cx.notify();
        }
    }
}
