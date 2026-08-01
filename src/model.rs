use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Todo {
    pub(crate) id: u64,
    pub(crate) title: String,
    pub(crate) completed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TodoState {
    next_id: u64,
    pub(crate) items: Vec<Todo>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ModelError {
    IdExhausted,
}

impl Default for TodoState {
    fn default() -> Self {
        Self {
            next_id: 1,
            items: Vec::new(),
        }
    }
}

impl TodoState {
    pub(crate) fn add(&mut self, title: &str) -> Result<bool, ModelError> {
        let title = title.trim();
        if title.is_empty() {
            return Ok(false);
        }

        let next_id = self.next_id.checked_add(1).ok_or(ModelError::IdExhausted)?;
        self.items.push(Todo {
            id: self.next_id,
            title: title.to_owned(),
            completed: false,
        });
        self.next_id = next_id;
        Ok(true)
    }

    pub(crate) fn toggle(&mut self, id: u64) -> bool {
        let Some(todo) = self.items.iter_mut().find(|todo| todo.id == id) else {
            return false;
        };
        todo.completed = !todo.completed;
        true
    }

    pub(crate) fn delete(&mut self, id: u64) -> bool {
        let previous_len = self.items.len();
        self.items.retain(|todo| todo.id != id);
        self.items.len() != previous_len
    }

    pub(crate) fn remaining_count(&self) -> usize {
        self.items.iter().filter(|todo| !todo.completed).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_trims_titles_and_assigns_monotonic_ids() {
        let mut state = TodoState::default();

        assert_eq!(state.add("  Buy milk  "), Ok(true));
        assert_eq!(state.add("Walk the dog"), Ok(true));

        assert_eq!(state.items[0].id, 1);
        assert_eq!(state.items[0].title, "Buy milk");
        assert_eq!(state.items[1].id, 2);
    }

    #[test]
    fn add_rejects_blank_titles_without_consuming_an_id() {
        let mut state = TodoState::default();

        assert_eq!(state.add("   "), Ok(false));
        assert_eq!(state.add("First"), Ok(true));

        assert_eq!(state.items.len(), 1);
        assert_eq!(state.items[0].id, 1);
    }

    #[test]
    fn toggle_and_delete_report_whether_the_todo_exists() {
        let mut state = TodoState::default();
        state.add("Ship it").unwrap();

        assert!(state.toggle(1));
        assert!(state.items[0].completed);
        assert_eq!(state.remaining_count(), 0);
        assert!(!state.toggle(99));

        assert!(state.delete(1));
        assert!(state.items.is_empty());
        assert!(!state.delete(1));
    }

    #[test]
    fn serialized_state_preserves_the_next_id_after_deletion() {
        let mut state = TodoState::default();
        state.add("One").unwrap();
        state.add("Two").unwrap();
        state.delete(2);

        let json = serde_json::to_string(&state).unwrap();
        let mut restored: TodoState = serde_json::from_str(&json).unwrap();
        restored.add("Three").unwrap();

        assert_eq!(restored.items.last().unwrap().id, 3);
    }

    #[test]
    fn add_reports_id_exhaustion_without_mutating_state() {
        let mut state = TodoState {
            next_id: u64::MAX,
            items: Vec::new(),
        };

        assert_eq!(state.add("Impossible"), Err(ModelError::IdExhausted));
        assert!(state.items.is_empty());
    }
}
