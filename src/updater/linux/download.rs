use super::super::signature::{public_key, verify_package};
use super::LinuxUpdate;
use anyhow::{Result, ensure};
use std::{io::Read, time::Duration};

const MAX_PACKAGE_BYTES: u64 = 256 * 1024 * 1024;

pub(super) fn download(update: &LinuxUpdate) -> Result<Vec<u8>> {
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
    Ok(bytes)
}

pub(super) fn verify(update: &LinuxUpdate, bytes: &[u8]) -> Result<()> {
    verify_package(bytes, &update.package.signature, public_key())
}
