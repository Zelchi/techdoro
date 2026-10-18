use super::{bootstrap, cli};
use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    if let Some(startup) = cli::prepare()? {
        bootstrap::start(startup)?;
    }
    Ok(())
}
