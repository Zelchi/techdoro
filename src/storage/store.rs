use super::decode;
use crate::domain::SavedState;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Storage {
    pub directory: PathBuf,
}

impl Storage {
    pub fn discover() -> Result<Self> {
        let directory = if let Some(path) = std::env::var_os("TECHDORO_DATA_DIR") {
            PathBuf::from(path)
        } else {
            ProjectDirs::from("com", "zelchi", "techdoro")
                .context("Could not locate the data directory")?
                .data_local_dir()
                .to_owned()
        };
        fs::create_dir_all(&directory).context("Could not create the data directory")?;
        Ok(Self { directory })
    }

    pub fn path(&self) -> PathBuf {
        self.directory.join("state.json")
    }

    pub fn load(&self) -> Result<SavedState> {
        match fs::read(self.path()) {
            Ok(bytes) => decode(&bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(SavedState::default()),
            Err(error) => Err(error).context("Could not read state.json"),
        }
    }

    pub fn save(&self, state: &SavedState) -> Result<()> {
        let mut temporary = tempfile::NamedTempFile::new_in(&self.directory)?;
        temporary.write_all(&serde_json::to_vec_pretty(state)?)?;
        temporary.as_file().sync_all()?;
        temporary
            .persist(self.path())
            .context("Could not save state.json")?;
        Ok(())
    }

    pub fn import(&self, path: &Path) -> Result<()> {
        let state = decode(&fs::read(path).context("Could not read the import file")?)?;
        if self.path().exists() {
            let backup = self.directory.join(format!(
                "state-before-import-{}.json",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_nanos()
            ));
            fs::copy(self.path(), backup)?;
        }
        self.save(&state)
    }
}
