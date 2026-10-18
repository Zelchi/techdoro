use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::path::Path;
const PUBLIC_KEY: &str = include_str!("../../assets/updater.pub");

pub(super) fn public_key() -> &'static str {
    option_env!("TECHDORO_UPDATER_PUBLIC_KEY")
        .filter(|key| !key.trim().is_empty())
        .unwrap_or(PUBLIC_KEY)
        .trim()
}

pub(super) fn verify_package(bytes: &[u8], signature: &str, public_key: &str) -> Result<()> {
    let key = String::from_utf8(STANDARD.decode(public_key.trim())?)?;
    let signature = String::from_utf8(STANDARD.decode(signature.trim())?)?;
    minisign_verify::PublicKey::decode(&key)?
        .verify(
            bytes,
            &minisign_verify::Signature::decode(&signature)?,
            true,
        )
        .context("The package signature is invalid")
}

pub fn verify_release(path: &Path, signature: &Path) -> Result<()> {
    verify_package(
        &std::fs::read(path)?,
        &std::fs::read_to_string(signature)?,
        public_key(),
    )
}
