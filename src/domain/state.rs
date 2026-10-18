use super::{Settings, Task};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct SavedState {
    pub schema_version: u32,
    pub settings: Settings,
    pub tasks: Vec<Task>,
}

impl Default for SavedState {
    fn default() -> Self {
        Self {
            schema_version: 1,
            settings: Settings::default(),
            tasks: Vec::new(),
        }
    }
}

impl SavedState {
    pub fn normalize(&mut self) {
        self.settings.normalize();
        let mut seen = std::collections::HashSet::new();
        self.tasks.retain_mut(|task| {
            task.text = task.text.trim().to_owned();
            !task.text.is_empty() && seen.insert(task.id)
        });
    }
}
