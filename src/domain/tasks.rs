use super::SavedState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

impl SavedState {
    pub fn add_task(&mut self, text: &str) -> bool {
        let text = text.trim();
        if text.is_empty() {
            return false;
        }
        let Some(id) = self
            .tasks
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
        else {
            return false;
        };
        self.tasks.push(Task {
            id,
            text: text.to_owned(),
            completed: false,
        });
        true
    }

    pub fn edit_task(&mut self, id: u64, text: &str) -> bool {
        let text = text.trim();
        if text.is_empty() {
            return false;
        }
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.text = text.to_owned();
            true
        } else {
            false
        }
    }

    pub fn reorder_task(&mut self, id: u64, target: u64) {
        let source_index = self.tasks.iter().position(|task| task.id == id);
        let target_index = self.tasks.iter().position(|task| task.id == target);
        if let (Some(source_index), Some(target_index)) = (source_index, target_index) {
            self.move_task(id, target, source_index < target_index);
        }
    }

    pub fn move_task(&mut self, id: u64, target: u64, after: bool) {
        if id == target || !self.tasks.iter().any(|t| t.id == target) {
            return;
        }
        let Some(index) = self.tasks.iter().position(|t| t.id == id) else {
            return;
        };
        let task = self.tasks.remove(index);
        let index = self.tasks.iter().position(|t| t.id == target).unwrap();
        self.tasks.insert(index + usize::from(after), task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropping_tasks_on_rows_preserves_the_destination_in_both_directions() {
        for (source, target, expected) in [
            (1, 2, [2, 1, 3, 4]),
            (1, 3, [2, 3, 1, 4]),
            (1, 4, [2, 3, 4, 1]),
            (4, 3, [1, 2, 4, 3]),
            (4, 2, [1, 4, 2, 3]),
            (4, 1, [4, 1, 2, 3]),
            (2, 2, [1, 2, 3, 4]),
            (5, 2, [1, 2, 3, 4]),
            (2, 5, [1, 2, 3, 4]),
        ] {
            let mut state = SavedState::default();
            for text in ["One", "Two", "Three", "Four"] {
                state.add_task(text);
            }
            state.reorder_task(source, target);
            assert_eq!(
                state.tasks.iter().map(|task| task.id).collect::<Vec<_>>(),
                expected,
                "moving {source} onto {target}"
            );
        }
    }

    #[test]
    fn tasks_reject_blank_text_and_keep_order() {
        let mut state = SavedState::default();
        assert!(!state.add_task("  "));
        state.add_task(" One ");
        state.add_task("Two");
        state.add_task("Three");
        state.move_task(1, 3, true);
        assert_eq!(
            state.tasks.iter().map(|t| t.id).collect::<Vec<_>>(),
            [2, 3, 1]
        );
        state.move_task(1, 2, false);
        assert_eq!(
            state.tasks.iter().map(|t| t.id).collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert!(!state.edit_task(1, " "));
        assert_eq!(state.tasks[0].text, "One");
    }
}
