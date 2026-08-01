use crate::app::AppStore;
use crate::theme;
use whisker::css::{
    AlignItems, AlignSelf, BorderStyle, Display, FlexDirection, FontWeight, JustifyContent,
    TextDecorationLine,
};
use whisker::prelude::*;
use whisker::runtime::view::Element;

#[component]
pub(crate) fn todo_row(id: u64) -> Element {
    let store = use_context::<AppStore>().expect("AppStore is provided by AppRoot");
    let title = computed(move || {
        store
            .state
            .get()
            .items
            .into_iter()
            .find(|todo| todo.id == id)
            .map(|todo| todo.title)
            .unwrap_or_default()
    });
    let completed = computed(move || {
        store
            .state
            .get()
            .items
            .iter()
            .find(|todo| todo.id == id)
            .is_some_and(|todo| todo.completed)
    });
    let toggle_label = computed(move || {
        let state = store.state.get();
        let Some(todo) = state.items.iter().find(|todo| todo.id == id) else {
            return "タスク".to_owned();
        };
        if todo.completed {
            format!("{}を未完了に戻す", todo.title)
        } else {
            format!("{}を完了にする", todo.title)
        }
    });
    let delete_label = computed(move || {
        let state = store.state.get();
        let title = state
            .items
            .iter()
            .find(|todo| todo.id == id)
            .map(|todo| todo.title.as_str())
            .unwrap_or("タスク");
        format!("{title}を削除")
    });
    let check_style = computed(move || {
        if completed.get() {
            css!(
                width: px(24),
                height: px(24),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin_left: px(8),
                margin_right: px(12),
                border_radius: px(12),
                background_color: theme::ACCENT,
            )
        } else {
            css!(
                width: px(24),
                height: px(24),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin_left: px(8),
                margin_right: px(12),
                border_radius: px(12),
                border_top_width: px(1),
                border_top_style: BorderStyle::Solid,
                border_top_color: theme::RULE_STRONG,
                border_right_width: px(1),
                border_right_style: BorderStyle::Solid,
                border_right_color: theme::RULE_STRONG,
                border_bottom_width: px(1),
                border_bottom_style: BorderStyle::Solid,
                border_bottom_color: theme::RULE_STRONG,
                border_left_width: px(1),
                border_left_style: BorderStyle::Solid,
                border_left_color: theme::RULE_STRONG,
                background_color: theme::BACKGROUND,
            )
        }
    });
    let title_style = computed(move || {
        if completed.get() {
            css!(
                flex_grow: 1.0,
                flex_shrink: 1.0,
                color: theme::MUTED,
                font_size: px(15),
                line_height: px(21),
                text_decoration_line: TextDecorationLine::LineThrough,
            )
        } else {
            css!(
                flex_grow: 1.0,
                flex_shrink: 1.0,
                color: theme::INK,
                font_size: px(15),
                line_height: px(21),
                text_decoration_line: TextDecorationLine::None,
            )
        }
    });

    render! {
        view(
            style: css!(
                min_height: px(70),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                border_bottom_width: px(1),
                border_bottom_style: BorderStyle::Solid,
                border_bottom_color: theme::RULE,
            ),
        ) {
            view(
                accessibility_element: true,
                accessibility_label: toggle_label,
                accessibility_trait: AccessibilityTrait::Button,
                user_interaction_enabled: true,
                on_tap: move |_| store.toggle(id),
                style: css!(
                    min_height: px(69),
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                ),
            ) {
                view(style: check_style) {
                    text(
                        value: computed(move || {
                            if completed.get() {
                                "✓".to_owned()
                            } else {
                                String::new()
                            }
                        }),
                        style: css!(
                            color: theme::ACCENT_INK,
                            font_size: px(14),
                            font_weight: FontWeight::Numeric(700),
                        ),
                    )
                }
                text(value: title, style: title_style, accessibility_elements_hidden: true)
            }
            view(
                accessibility_element: true,
                accessibility_label: delete_label,
                accessibility_trait: AccessibilityTrait::Button,
                user_interaction_enabled: true,
                hit_slop: "8px 8px 8px 8px",
                on_tap_catch: move |_| store.delete(id),
                style: css!(
                    width: px(48),
                    height: px(48),
                    align_self: AlignSelf::FlexStart,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                ),
            ) {
                text(
                    value: "消す",
                    style: css!(
                        color: theme::ERROR,
                        font_size: px(12),
                        font_weight: FontWeight::Numeric(600),
                    ),
                )
            }
        }
    }
}
