use anyhow::{Context, Result};
use fs2::FileExt;
use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};
use techdoro::storage::Storage;

pub struct Instance {
    lock: File,
    activation: PathBuf,
}

impl Instance {
    pub fn acquire(storage: &Storage, activate_existing: bool) -> Result<Option<Self>> {
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(storage.directory.join("instance.lock"))?;
        let activation = storage.directory.join("activate");
        match FileExt::try_lock_exclusive(&lock) {
            Ok(()) => {
                let _ = fs::remove_file(&activation);
                Ok(Some(Self { lock, activation }))
            }
            Err(error) if error.raw_os_error() == fs2::lock_contended_error().raw_os_error() => {
                if activate_existing {
                    fs::write(activation, b"show")?;
                }
                Ok(None)
            }
            Err(error) => Err(error).context("Could not acquire the instance lock"),
        }
    }

    pub fn take_activation(&self) -> bool {
        fs::remove_file(&self.activation).is_ok()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // Duplicated or inherited descriptors can outlive this instance.
        // Explicitly unlock instead of waiting for every descriptor to close.
        if let Err(error) = FileExt::unlock(&self.lock) {
            eprintln!("Could not release the instance lock: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_instance_requests_activation_and_lock_is_released_on_exit() {
        let directory = tempfile::tempdir().unwrap();
        let storage = Storage {
            directory: directory.path().to_owned(),
        };
        let first = Instance::acquire(&storage, true).unwrap().unwrap();
        assert!(Instance::acquire(&storage, true).unwrap().is_none());
        assert!(first.take_activation());
        assert!(!first.take_activation());
        drop(first);
        assert!(Instance::acquire(&storage, true).unwrap().is_some());
    }

    #[test]
    fn import_cannot_acquire_a_running_instances_data() {
        let directory = tempfile::tempdir().unwrap();
        let storage = Storage {
            directory: directory.path().to_owned(),
        };
        let first = Instance::acquire(&storage, true).unwrap().unwrap();
        assert!(Instance::acquire(&storage, false).unwrap().is_none());
        assert!(!first.take_activation());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn dropping_instance_releases_lock_with_a_duplicate_descriptor() {
        let directory = tempfile::tempdir().unwrap();
        let storage = Storage {
            directory: directory.path().to_owned(),
        };
        let first = Instance::acquire(&storage, false).unwrap().unwrap();
        // A concurrently spawned process can inherit the same open file description
        // between fork and exec. Keep a duplicate alive to reproduce that condition.
        let duplicate = first.lock.try_clone().unwrap();
        assert!(Instance::acquire(&storage, false).unwrap().is_none());

        drop(first);
        let second = Instance::acquire(&storage, false)
            .unwrap()
            .expect("Dropping the owner must release its lock immediately");
        drop(duplicate);
        assert!(Instance::acquire(&storage, false).unwrap().is_none());
        drop(second);
        assert!(Instance::acquire(&storage, false).unwrap().is_some());
    }

    #[test]
    fn another_process_activates_the_existing_instance() {
        const PROBE_DIRECTORY: &str = "TECHDORO_INSTANCE_TEST_DIRECTORY";
        if let Some(directory) = std::env::var_os(PROBE_DIRECTORY) {
            let storage = Storage {
                directory: directory.into(),
            };
            assert!(Instance::acquire(&storage, true).unwrap().is_none());
            return;
        }

        let directory = tempfile::tempdir().unwrap();
        let storage = Storage {
            directory: directory.path().to_owned(),
        };
        let first = Instance::acquire(&storage, true).unwrap().unwrap();
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "desktop::instance::tests::another_process_activates_the_existing_instance",
            ])
            .env(PROBE_DIRECTORY, directory.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "Child instance failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        assert!(first.take_activation());
        assert!(!first.take_activation());
        assert!(Instance::acquire(&storage, false).unwrap().is_none());
    }
}
