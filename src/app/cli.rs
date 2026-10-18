use crate::{desktop::Instance, updater};
use anyhow::{Context, Result, bail};
use std::path::Path;
use techdoro::{domain::SavedState, storage::Storage};

pub(super) struct Startup {
    pub saved: SavedState,
    pub storage: Storage,
    pub instance: Instance,
}

pub(super) fn prepare() -> Result<Option<Startup>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 3 && args[0] == "--verify-update" {
        updater::verify_release(Path::new(&args[1]), Path::new(&args[2]))?;
        return Ok(None);
    }
    if args
        .first()
        .is_some_and(|arg| arg == "--help" || arg == "-h")
    {
        println!(
            "Techdoro {}\n\ntechdoro\ntechdoro --import <file.json>\ntechdoro --export <file.json>\ntechdoro --data-dir",
            env!("CARGO_PKG_VERSION")
        );
        return Ok(None);
    }
    if args
        .first()
        .is_some_and(|arg| arg == "--version" || arg == "-V")
    {
        println!("Techdoro {}", env!("CARGO_PKG_VERSION"));
        return Ok(None);
    }

    let storage = Storage::discover()?;
    if args.first().is_some_and(|arg| arg == "--data-dir") {
        println!("{}", storage.directory.display());
        return Ok(None);
    }

    let Some(instance) = Instance::acquire(&storage, args.is_empty())? else {
        if !args.is_empty() {
            bail!("Quit Techdoro from the tray menu before importing or exporting data");
        }
        return Ok(None);
    };
    if !args.is_empty() {
        if args.len() != 2 {
            bail!("Use --help to see the available commands");
        }
        let path = Path::new(&args[1]);
        if args[0] == "--import" {
            storage.import(path)?;
            println!("Data imported to {}", storage.path().display());
        } else if args[0] == "--export" {
            use std::io::Write;
            let state = storage.load()?;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .context("Choose an export file that does not already exist")?;
            file.write_all(&serde_json::to_vec_pretty(&state)?)?;
            file.sync_all()?;
        } else {
            bail!("Unknown command. Use --help");
        }
        return Ok(None);
    }

    let saved = storage
        .load()
        .context("Existing data was not changed. Fix or restore state.json")?;
    Ok(Some(Startup {
        saved,
        storage,
        instance,
    }))
}
