use whisker::css::{Color, Length};

pub(crate) const BACKGROUND: Color = Color::hex(0xF2EFE6);
pub(crate) const SURFACE: Color = Color::hex(0xFAF8F2);
pub(crate) const INK: Color = Color::hex(0x20241F);
pub(crate) const MUTED: Color = Color::hex(0x656B64);
pub(crate) const RULE: Color = Color::hex(0xCFD0C6);
pub(crate) const RULE_STRONG: Color = Color::hex(0x7F877F);
pub(crate) const ACCENT: Color = Color::hex(0x3F6655);
pub(crate) const ACCENT_INK: Color = Color::hex(0xFAF8F2);
pub(crate) const ERROR: Color = Color::hex(0x984438);
pub(crate) const ERROR_SOFT: Color = Color::hex(0xF1E1DC);

pub(crate) const RADIUS: Length = Length::Px(6.0);

pub(crate) const INPUT_PLACEHOLDER: &str = "#656b64";
pub(crate) const INPUT_CARET: &str = "#3f6655";
pub(crate) const INPUT_SELECTION: &str = "#c9d9cf";
pub(crate) const INPUT_STYLE: &str = "flex-grow: 1; flex-shrink: 1; min-height: 48px; color: #20241f; font-size: 16px; padding-right: 12px;";
