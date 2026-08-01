use crate::app::AppStore;
use crate::components::TodoRow;
use crate::model::Todo;
use crate::theme;
use whisker::css::{AlignItems, BorderStyle, Display, FlexDirection, FontWeight, JustifyContent};
use whisker::prelude::*;
use whisker::runtime::view::Element;
use whisker_input::{Input, ReturnKey};
use whisker_safe_area::safe_area_insets;

// Hallmark · pre-emit critique: P5 H5 E4 S5 R5 V4
// Hallmark · genre: editorial · macrostructure: quiet ledger · theme: Almanac

#[component]
pub(crate) fn app_screen() -> Element {
    let store = use_context::<AppStore>().expect("AppStore is provided by AppRoot");
    let insets = safe_area_insets();
    let screen_style = computed(move || {
        let insets = insets.get();
        css!(
            flex_grow: 1.0,
            flex_shrink: 1.0,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding_top: px(insets.top as f32 + 20.0),
            padding_bottom: px(insets.bottom as f32 + 8.0),
            padding_left: px(insets.leading as f32 + 24.0),
            padding_right: px(insets.trailing as f32 + 24.0),
            background_color: theme::BACKGROUND,
        )
    });

    render! {
        view(style: screen_style) {
            Header
            Show(when: move || !store.storage_locked.get()) {
                TodoEditor(store: store)
            }
            Show(when: move || store.error.get().is_some()) {
                ErrorBanner(store: store)
            }
            Show(when: move || store.storage_locked.get()) {
                RecoveryPanel(store: store)
            }
            Show(when: move || !store.storage_locked.get()) {
                TodoList(store: store)
                Footer(store: store)
            }
        }
    }
}

#[component]
fn header() -> Element {
    render! {
        view(
            style: css!(
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin_bottom: px(28),
            ),
        ) {
            text(
                value: "今日",
                style: css!(
                    color: theme::INK,
                    font_size: px(40),
                    font_weight: FontWeight::Numeric(760),
                    line_height: px(46),
                    letter_spacing: px(-1.2),
                ),
            )
            text(
                value: "やることを、ひとつずつ。",
                style: css!(color: theme::MUTED, font_size: px(14), margin_top: px(4)),
            )
            view(
                style: css!(
                    width: px(32),
                    height: px(2),
                    margin_top: px(16),
                    background_color: theme::ACCENT,
                ),
            )
        }
    }
}

#[component]
fn todo_editor(store: AppStore) -> Element {
    let add_from_draft = move || {
        let title = store.draft.get();
        store.submit(&title);
    };

    render! {
        view(
            style: css!(
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin_bottom: px(16),
            ),
        ) {
            text(
                value: "タスクを追加",
                style: css!(
                    color: theme::MUTED,
                    font_size: px(12),
                    font_weight: FontWeight::Numeric(600),
                    letter_spacing: px(0.4),
                    margin_bottom: px(8),
                ),
            )
            view(
                style: css!(
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    min_height: px(58),
                    border_top_width: px(1),
                    border_top_style: BorderStyle::Solid,
                    border_top_color: theme::RULE_STRONG,
                    border_bottom_width: px(1),
                    border_bottom_style: BorderStyle::Solid,
                    border_bottom_color: theme::RULE,
                ),
            ) {
                Input(
                    text: store.draft,
                    on_input: move |_value: String| store.clear_transient_error(),
                    on_submit: move |value: String| store.submit(&value),
                    placeholder: "例：牛乳を買う",
                    placeholder_color: theme::INPUT_PLACEHOLDER,
                    caret_color: theme::INPUT_CARET,
                    selection_color: theme::INPUT_SELECTION,
                    max_length: 120u32,
                    return_key: ReturnKey::Done,
                    style: theme::INPUT_STYLE,
                )
                view(
                    accessibility_element: true,
                    accessibility_label: "入力したタスクを追加",
                    accessibility_trait: AccessibilityTrait::Button,
                    user_interaction_enabled: true,
                    on_tap: move |_| add_from_draft(),
                    style: css!(
                        width: px(48),
                        height: px(48),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border_radius: px(24),
                        background_color: theme::ACCENT,
                    ),
                ) {
                    text(
                        value: "+",
                        style: css!(
                            color: theme::ACCENT_INK,
                            font_size: px(25),
                            font_weight: FontWeight::Numeric(500),
                            line_height: px(26),
                        ),
                    )
                }
            }
        }
    }
}

#[component]
fn error_banner(store: AppStore) -> Element {
    render! {
        view(
            accessibility_element: true,
            accessibility_label: computed(move || store.error.get().unwrap_or_default()),
            style: css!(
                padding_top: px(12),
                padding_bottom: px(12),
                margin_bottom: px(12),
                border_top_width: px(1),
                border_top_style: BorderStyle::Solid,
                border_top_color: theme::ERROR,
                border_bottom_width: px(1),
                border_bottom_style: BorderStyle::Solid,
                border_bottom_color: theme::ERROR,
                background_color: theme::ERROR_SOFT,
            ),
        ) {
            text(
                value: computed(move || store.error.get().unwrap_or_default()),
                style: css!(color: theme::ERROR, font_size: px(13), line_height: px(19)),
            )
        }
    }
}

#[component]
fn recovery_panel(store: AppStore) -> Element {
    render! {
        view(
            style: css!(
                padding: px(16),
                border_top_width: px(1),
                border_top_style: BorderStyle::Solid,
                border_top_color: theme::RULE_STRONG,
                border_bottom_width: px(1),
                border_bottom_style: BorderStyle::Solid,
                border_bottom_color: theme::RULE_STRONG,
                background_color: theme::SURFACE,
            ),
        ) {
            text(
                value: "保存データをリセット",
                style: css!(color: theme::INK, font_size: px(18), font_weight: FontWeight::Numeric(700)),
            )
            text(
                value: "端末に残っている Todo は削除されます。壊れたデータを上書きせず、確認してから復旧できます。",
                style: css!(
                    color: theme::MUTED,
                    font_size: px(14),
                    line_height: px(21),
                    margin_top: px(8),
                    margin_bottom: px(16),
                ),
            )
            view(
                accessibility_element: true,
                accessibility_label: "保存データをリセットして Todo を再開",
                accessibility_trait: AccessibilityTrait::Button,
                user_interaction_enabled: true,
                on_tap: move |_| store.reset_local_data(),
                style: css!(
                    min_height: px(48),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border_radius: theme::RADIUS,
                    background_color: theme::ERROR,
                ),
            ) {
                text(
                    value: "リセットして再開",
                    style: css!(
                        color: theme::ACCENT_INK,
                        font_size: px(14),
                        font_weight: FontWeight::Numeric(700),
                    ),
                )
            }
        }
    }
}

#[component]
fn todo_list(store: AppStore) -> Element {
    render! {
        scroll_view(
            scroll_orientation: ScrollOrientation::Vertical,
            scroll_bar_enable: false,
            bounces: true,
            style: css!(flex_grow: 1.0, flex_shrink: 1.0, width: percent(100)),
        ) {
            view(
                style: css!(
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    padding_bottom: px(12),
                ),
            ) {
                Show(when: move || store.state.get().items.is_empty()) {
                    EmptyState
                }
                ForEach(
                    each: move || store.state.get().items,
                    key: |todo: &Todo| todo.id,
                    children: |todo: Todo| render! { TodoRow(id: todo.id) },
                )
            }
        }
    }
}

#[component]
fn empty_state() -> Element {
    render! {
        view(
            style: css!(
                min_height: px(180),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                padding_bottom: px(24),
                border_top_width: px(1),
                border_top_style: BorderStyle::Solid,
                border_top_color: theme::RULE,
            ),
        ) {
            text(
                value: "今日はまだ、まっさらです。",
                style: css!(color: theme::INK, font_size: px(19), font_weight: FontWeight::Numeric(650)),
            )
            text(
                value: "思いついたことを、上の欄に書いてください。",
                style: css!(color: theme::MUTED, font_size: px(13), line_height: px(19), margin_top: px(8)),
            )
        }
    }
}

#[component]
fn footer(store: AppStore) -> Element {
    render! {
        view(
            style: css!(
                min_height: px(42),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding_top: px(12),
                border_top_width: px(1),
                border_top_style: BorderStyle::Solid,
                border_top_color: theme::RULE_STRONG,
            ),
        ) {
            text(
                value: computed(move || {
                    let state = store.state.get();
                    let completed = state.items.len() - state.remaining_count();
                    format!("未完了 {}　・　完了 {completed}", state.remaining_count())
                }),
                style: css!(
                    color: theme::MUTED,
                    font_size: px(12),
                    font_weight: FontWeight::Numeric(600),
                    letter_spacing: px(0.3),
                ),
            )
        }
    }
}
