use super::*;
use crate::domain::SavedState;
use std::fs;

#[test]
fn imports_tauri_local_storage_strings() {
    let state = decode(br#"{"timeLongMax":"50","volume":"500","tarefas":"[{\"id\":1,\"text\":\"Estudar\",\"completed\":true}]"}"#).unwrap();
    assert_eq!(state.settings.focus_minutes, 50);
    assert_eq!(state.settings.volume, 100);
    assert_eq!(state.tasks[0].text, "Estudar");
    assert!(state.tasks[0].completed);
}

#[test]
fn imports_a_tauri_profile_without_saved_tasks() {
    let state = decode(br#"{"timeLongMax":"25","tarefas":null,"volume":null}"#).unwrap();
    assert!(state.tasks.is_empty());
    assert_eq!(state.settings.volume, 30);
}

#[test]
fn roundtrip_and_import_backup() {
    let dir = tempfile::tempdir().unwrap();
    let storage = Storage {
        directory: dir.path().to_owned(),
    };
    let mut state = SavedState::default();
    state.add_task("Preservar");
    storage.save(&state).unwrap();
    assert_eq!(storage.load().unwrap(), state);
    let import = dir.path().join("import.json");
    fs::write(&import, br#"{"schema_version":1}"#).unwrap();
    storage.import(&import).unwrap();
    assert!(storage.load().unwrap().tasks.is_empty());
    assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 3);
}

#[test]
fn corrupt_or_future_data_is_not_silently_reset() {
    assert!(decode(b"broken").is_err());
    assert!(decode(br#"{"schema_version":2}"#).is_err());
    assert!(decode(br#"{}"#).is_err());
}
