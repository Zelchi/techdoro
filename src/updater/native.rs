use super::{AvailableUpdate, ENDPOINT, UpdateMessage, signature::public_key};
use anyhow::Result;
use cargo_packager_updater::{Config, UpdaterBuilder, semver::Version};
use std::time::Duration;

pub(super) fn check_native() -> Result<UpdateMessage> {
    let config = Config {
        endpoints: vec![ENDPOINT.parse()?],
        pubkey: public_key().to_owned(),
        ..Default::default()
    };
    let update = UpdaterBuilder::new(Version::parse(env!("CARGO_PKG_VERSION"))?, config)
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
