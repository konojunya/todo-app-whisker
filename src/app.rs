use crate::components::AppScreen;
use crate::model::{ModelError, TodoState};
use crate::repository;
use whisker::prelude::*;
use whisker::runtime::view::Element;

#[derive(Clone, Copy)]
pub(crate) struct AppStore {
    pub(crate) state: RwSignal<TodoState>,
    pub(crate) draft: RwSignal<String>,
    pub(crate) error: RwSignal<Option<String>>,
    pub(crate) storage_locked: RwSignal<bool>,
}

impl AppStore {
    fn new() -> Self {
        let (state, error, storage_locked) = match repository::load_state() {
            Ok(state) => (state, None, false),
            Err(error) => {
                eprintln!("failed to load todo state: {error}");
                (
                    TodoState::default(),
                    Some(
                        "保存データを読み込めませんでした。リセットすると、もう一度使えます。"
                            .to_owned(),
                    ),
                    true,
                )
            }
        };

        Self {
            state: RwSignal::new(state),
            draft: RwSignal::new(String::new()),
            error: RwSignal::new(error),
            storage_locked: RwSignal::new(storage_locked),
        }
    }

    pub(crate) fn submit(self, title: &str) {
        if self.storage_locked.get() {
            return;
        }

        let mut candidate = self.state.get();
        match candidate.add(title) {
            Ok(false) => self
                .error
                .set(Some("タスク名を入力してください。".to_owned())),
            Ok(true) => {
                if self.persist(candidate) {
                    self.draft.set(String::new());
                }
            }
            Err(ModelError::IdExhausted) => self.error.set(Some(
                "これ以上タスクを追加できません。不要なタスクを削除してください。".to_owned(),
            )),
        }
    }

    pub(crate) fn toggle(self, id: u64) {
        if self.storage_locked.get() {
            return;
        }

        let mut candidate = self.state.get();
        if candidate.toggle(id) {
            self.persist(candidate);
        }
    }

    pub(crate) fn delete(self, id: u64) {
        if self.storage_locked.get() {
            return;
        }

        let mut candidate = self.state.get();
        if candidate.delete(id) {
            self.persist(candidate);
        }
    }

    pub(crate) fn clear_transient_error(self) {
        if !self.storage_locked.get() {
            self.error.set(None);
        }
    }

    pub(crate) fn reset_local_data(self) {
        match repository::clear_state() {
            Ok(()) => {
                self.state.set(TodoState::default());
                self.draft.set(String::new());
                self.error.set(None);
                self.storage_locked.set(false);
            }
            Err(error) => {
                eprintln!("failed to reset todo state: {error}");
                self.error.set(Some(
                    "保存データをリセットできませんでした。アプリを再起動してお試しください。"
                        .to_owned(),
                ));
            }
        }
    }

    fn persist(self, candidate: TodoState) -> bool {
        match repository::save_state(&candidate) {
            Ok(()) => {
                self.state.set(candidate);
                self.error.set(None);
                true
            }
            Err(error) => {
                eprintln!("failed to save todo state: {error}");
                self.error.set(Some(
                    "変更を保存できませんでした。通信環境ではなく、端末の保存領域をご確認ください。"
                        .to_owned(),
                ));
                false
            }
        }
    }
}

#[component]
pub(crate) fn app_root() -> Element {
    let store = AppStore::new();
    provide_context(store);

    render! {
        AppScreen
    }
}
