use super::{AvailableUpdate, ENDPOINT, UpdateMessage, signature::public_key};
use anyhow::Result;
use cargo_packager_updater::{
    Config, UpdaterBuilder, WindowsConfig, WindowsUpdateInstallMode, semver::Version,
};
use std::time::Duration;

pub(super) fn native_config() -> Result<Config> {
    Ok(Config {
        endpoints: vec![ENDPOINT.parse()?],
        pubkey: public_key().to_owned(),
        windows: Some(WindowsConfig {
            install_mode: Some(WindowsUpdateInstallMode::Passive),
            ..Default::default()
        }),
    })
}

pub(super) fn check_native() -> Result<UpdateMessage> {
    let update = UpdaterBuilder::new(Version::parse(env!("CARGO_PKG_VERSION"))?, native_config()?)
        .timeout(Duration::from_secs(30))
        .build()?
        .check()?;
    Ok(match update {
        Some(update) => {
            UpdateMessage::Available(Box::new(AvailableUpdate::Native(Box::new(update))))
        }
        None => UpdateMessage::Current,
    })
}
