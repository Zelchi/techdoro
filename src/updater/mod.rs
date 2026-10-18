mod linux;
mod native;
mod signature;
#[cfg(test)]
mod tests;

use cargo_packager_updater::Update;
use linux::{LinuxUpdate, check_linux, install_linux};
use native::check_native;
pub(crate) use signature::verify_release;
use std::sync::mpsc;

const ENDPOINT: &str =
    "https://github.com/Zelchi/Techdoro/releases/latest/download/native-latest.json";

#[derive(Clone)]
pub enum AvailableUpdate {
    Native(Box<Update>),
    Linux(LinuxUpdate),
}

impl AvailableUpdate {
    pub fn version(&self) -> &str {
        match self {
            Self::Native(update) => &update.version,
            Self::Linux(update) => &update.version,
        }
    }
}

pub enum UpdateMessage {
    Available(Box<AvailableUpdate>),
    Current,
    Installed,
    CheckFailed(String),
    InstallFailed(String),
    NotPackaged,
    Status(&'static str),
}

pub fn check(sender: mpsc::Sender<UpdateMessage>) {
    std::thread::spawn(move || {
        let result = if cfg!(target_os = "linux") {
            check_linux()
        } else {
            check_native()
        };
        let message = match result {
            Ok(message) => message,
            Err(error) => UpdateMessage::CheckFailed(error.to_string()),
        };
        let _ = sender.send(message);
    });
}

pub fn install(update: AvailableUpdate, sender: mpsc::Sender<UpdateMessage>) {
    std::thread::spawn(move || {
        let result = match update {
            AvailableUpdate::Native(update) => {
                update.download_and_install().map_err(anyhow::Error::from)
            }
            AvailableUpdate::Linux(update) => install_linux(update, &sender),
        };
        let message = match result {
            Ok(()) => UpdateMessage::Installed,
            Err(error) => UpdateMessage::InstallFailed(error.to_string()),
        };
        let _ = sender.send(message);
    });
}
