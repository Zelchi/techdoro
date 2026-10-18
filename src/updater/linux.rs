use super::{
    AvailableUpdate, ENDPOINT, UpdateMessage,
    signature::{public_key, verify_package},
};
use anyhow::{Context, Result, bail, ensure};
use cargo_packager_updater::semver::Version;
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
    process::Command,
    sync::mpsc,
    time::Duration,
};
const MAX_PACKAGE_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PackageFormat {
    Deb,
    Rpm,
}

impl PackageFormat {
    fn name(self) -> &'static str {
        match self {
            Self::Deb => "deb",
            Self::Rpm => "rpm",
        }
    }

    fn manager(self) -> &'static str {
        match self {
            Self::Deb => "/usr/bin/apt-get",
            Self::Rpm => "/usr/bin/dnf",
        }
    }
}

#[derive(Clone, Deserialize)]
pub(super) struct Package {
    pub(super) url: String,
    pub(super) signature: String,
}

#[derive(Deserialize)]
pub(super) struct LinuxManifest {
    pub(super) version: String,
    pub(super) packages: HashMap<String, HashMap<String, Package>>,
}

#[derive(Clone)]
pub struct LinuxUpdate {
    pub(super) version: String,
    pub(super) package: Package,
    pub(super) format: PackageFormat,
}

pub(super) fn check_linux() -> Result<UpdateMessage> {
    let Some(format) = installed_package_format(&std::env::current_exe()?) else {
        return Ok(UpdateMessage::NotPackaged);
    };
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    let response = client.get(ENDPOINT).send()?.error_for_status()?;
    let mut bytes = Vec::new();
    response.take(1024 * 1024).read_to_end(&mut bytes)?;
    let manifest: LinuxManifest = serde_json::from_slice(&bytes)?;
    let current = Version::parse(env!("CARGO_PKG_VERSION"))?;
    Ok(
        match select_linux_update(manifest, format, &current, std::env::consts::ARCH)? {
            Some(update) => UpdateMessage::Available(Box::new(AvailableUpdate::Linux(update))),
            None => UpdateMessage::Current,
        },
    )
}

pub(super) fn select_linux_update(
    manifest: LinuxManifest,
    format: PackageFormat,
    current: &Version,
    arch: &str,
) -> Result<Option<LinuxUpdate>> {
    let version = Version::parse(manifest.version.trim_start_matches('v'))?;
    if version <= *current {
        return Ok(None);
    }
    let package = manifest
        .packages
        .get(&format!("linux-{arch}"))
        .and_then(|packages| packages.get(format.name()))
        .context("No package is available for this architecture")?
        .clone();
    let url = reqwest::Url::parse(&package.url)?;
    ensure!(url.scheme() == "https", "Updates must use HTTPS");
    ensure!(
        url.path().ends_with(&format!(".{}", format.name())),
        "Incorrect update package format"
    );
    ensure!(!package.signature.is_empty(), "Missing update signature");
    Ok(Some(LinuxUpdate {
        version: version.to_string(),
        package,
        format,
    }))
}

fn installed_package_format(executable: &Path) -> Option<PackageFormat> {
    if let Ok(output) = Command::new("/usr/bin/dpkg-query")
        .arg("-S")
        .arg(executable)
        .output()
        && output.status.success()
        && String::from_utf8_lossy(&output.stdout)
            .lines()
            .any(|line| line.starts_with("techdoro:") || line.starts_with("techdoro:amd64:"))
    {
        return Some(PackageFormat::Deb);
    }
    if let Ok(output) = Command::new("/usr/bin/rpm")
        .args(["-qf", "--qf", "%{NAME}"])
        .arg(executable)
        .output()
        && output.status.success()
        && String::from_utf8_lossy(&output.stdout).trim() == "techdoro"
    {
        return Some(PackageFormat::Rpm);
    }
    None
}

pub(super) fn install_linux(
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
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(180))
        .build()?;
    let response = client.get(&update.package.url).send()?.error_for_status()?;
    if let Some(size) = response.content_length() {
        ensure!(size <= MAX_PACKAGE_BYTES, "Update package is too large");
    }
    let mut bytes = Vec::new();
    response
        .take(MAX_PACKAGE_BYTES + 1)
        .read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 <= MAX_PACKAGE_BYTES,
        "Update package is too large"
    );
    let _ = sender.send(UpdateMessage::Status("Verifying signature..."));
    verify_package(&bytes, &update.package.signature, public_key())?;
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

pub(super) fn installation_command(format: PackageFormat, path: &Path) -> Command {
    let mut command = Command::new("/usr/bin/pkexec");
    command
        .arg(format.manager())
        .args(["install", "-y"])
        .arg(path);
    command
}

fn run_package_manager(format: PackageFormat, path: &Path) -> Result<()> {
    let output = installation_command(format, path)
        .output()
        .context("Could not start the package manager")?;
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("Installation canceled or failed: {}", error.trim());
    }
    Ok(())
}
