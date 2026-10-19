use super::super::UpdateMessage;
use super::{LinuxUpdate, download::download, download::verify, package::run_package_manager};
use anyhow::{Result, ensure};
use std::{io::Write, path::Path, sync::mpsc};

pub(in crate::updater) fn install_linux(
    update: LinuxUpdate,
    sender: &mpsc::Sender<UpdateMessage>,
) -> Result<()> {
    ensure!(
        Path::new("/usr/bin/pkexec").exists(),
        "Install polkit to authorize the update"
    );
    ensure!(
        Path::new(update.format.manager()).exists(),
        "Package manager unavailable"
    );
    let _ = sender.send(UpdateMessage::Status("Downloading update..."));
    let bytes = download(&update)?;
    let _ = sender.send(UpdateMessage::Status("Verifying signature..."));
    verify(&update, &bytes)?;
    let directory = tempfile::tempdir()?;
    let path = directory
        .path()
        .join(format!("techdoro.{}", update.format.name()));
    let mut file = std::fs::File::create(&path)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    drop(file);
    let _ = sender.send(UpdateMessage::Status(
        "Installing: authorize in the system prompt...",
    ));
    run_package_manager(update.format, &path)?;
    Ok(())
}
