use super::super::{AvailableUpdate, ENDPOINT, UpdateMessage};
use super::{LinuxManifest, LinuxUpdate, PackageFormat};
use anyhow::{Context, Result, ensure};
use cargo_packager_updater::semver::Version;
use std::{io::Read, time::Duration};

pub(in crate::updater) fn check_linux() -> Result<UpdateMessage> {
    let Some(format) = super::package::installed_package_format(&std::env::current_exe()?) else {
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

pub(in crate::updater) fn select_linux_update(
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
