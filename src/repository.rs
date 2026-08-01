use std::fmt;

use crate::model::TodoState;
use whisker_local_store::WhiskerLocalStore;

const STATE_KEY: &str = "todos.state.v1";

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RepositoryError {
    InvalidData(String),
    Storage(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidData(message) => write!(formatter, "invalid local data: {message}"),
            Self::Storage(message) => write!(formatter, "local storage failed: {message}"),
        }
    }
}

pub(crate) fn encode_state(state: &TodoState) -> Result<String, RepositoryError> {
    serde_json::to_string(state).map_err(|error| RepositoryError::InvalidData(error.to_string()))
}

pub(crate) fn decode_state(raw: &str) -> Result<TodoState, RepositoryError> {
    serde_json::from_str(raw).map_err(|error| RepositoryError::InvalidData(error.to_string()))
}

pub(crate) fn load_state() -> Result<TodoState, RepositoryError> {
    let Some(raw) = WhiskerLocalStore::load(STATE_KEY.to_owned())
        .map_err(|error| RepositoryError::Storage(error.to_string()))?
    else {
        return Ok(TodoState::default());
    };

    decode_state(&raw)
}

pub(crate) fn save_state(state: &TodoState) -> Result<(), RepositoryError> {
    let raw = encode_state(state)?;
    let saved = WhiskerLocalStore::save(STATE_KEY.to_owned(), raw)
        .map_err(|error| RepositoryError::Storage(error.to_string()))?;
    if saved {
        Ok(())
    } else {
        Err(RepositoryError::Storage(
            "platform store reported an unsuccessful save".to_owned(),
        ))
    }
}

pub(crate) fn clear_state() -> Result<(), RepositoryError> {
    WhiskerLocalStore::remove(STATE_KEY.to_owned())
        .map_err(|error| RepositoryError::Storage(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec_round_trip_preserves_todos_and_next_id() {
        let mut state = TodoState::default();
        state.add("First").unwrap();
        state.add("Second").unwrap();
        state.delete(2);

        let raw = encode_state(&state).unwrap();
        let mut restored = decode_state(&raw).unwrap();
        restored.add("Third").unwrap();

        assert_eq!(restored.items.len(), 2);
        assert_eq!(restored.items[1].id, 3);
    }

    #[test]
    fn decode_reports_malformed_data_without_replacing_it() {
        let error = decode_state("not-json").unwrap_err();

        assert!(matches!(error, RepositoryError::InvalidData(_)));
    }
}
