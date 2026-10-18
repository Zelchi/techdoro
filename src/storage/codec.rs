use crate::domain::SavedState;
use anyhow::{Context, Result, bail};
use serde_json::Value;

pub fn decode(bytes: &[u8]) -> Result<SavedState> {
    let value: Value = serde_json::from_slice(bytes).context("Invalid data JSON")?;
    if !value.is_object() {
        bail!("The file must contain a JSON object");
    }
    let mut state = if value.get("schema_version").is_some() {
        let state: SavedState = serde_json::from_value(value)?;
        if state.schema_version != 1 {
            bail!("Unsupported data version: {}", state.schema_version);
        }
        state
    } else if value.get("tarefas").is_some() || value.get("timeLongMax").is_some() {
        let mut state = SavedState::default();
        for (key, field) in [
            ("volume", &mut state.settings.volume),
            ("timeLongMax", &mut state.settings.focus_minutes),
            ("timeShortMax", &mut state.settings.short_break_minutes),
            ("timeFinalMax", &mut state.settings.long_break_minutes),
            (
                "cyclesBeforeFinal",
                &mut state.settings.cycles_before_long_break,
            ),
        ] {
            if let Some(value) = value.get(key) {
                let parsed = value
                    .as_u64()
                    .and_then(|n| u32::try_from(n).ok())
                    .or_else(|| value.as_str().and_then(|s| s.parse().ok()));
                if let Some(parsed) = parsed {
                    *field = parsed;
                }
            }
        }
        if let Some(tasks) = value.get("tarefas").filter(|tasks| !tasks.is_null()) {
            state.tasks = if let Some(text) = tasks.as_str() {
                serde_json::from_str(text)?
            } else {
                serde_json::from_value(tasks.clone())?
            };
        }
        state
    } else {
        bail!("Unknown format: use state.json or a Techdoro 3 export");
    };
    state.normalize();
    Ok(state)
}
