//! Common Unicode symbols and icons useful for CLI output, such as
//! checkmarks, crosses, arrows, and spinner frame sets.

/// A checkmark symbol (✔), useful for success messages.
pub const CHECK: &str = "✔";
/// A cross/failure symbol (✖), useful for error messages.
pub const CROSS: &str = "✖";
/// A warning triangle symbol (⚠).
pub const WARNING: &str = "⚠";
/// An information symbol (ℹ).
pub const INFO: &str = "ℹ";
/// A right-pointing arrow (➜), useful as a prompt indicator.
pub const ARROW_RIGHT: &str = "➜";
/// A bullet point (•).
pub const BULLET: &str = "•";
/// A star symbol (★).
pub const STAR: &str = "★";
/// An hourglass symbol (⌛).
pub const HOURGLASS: &str = "⌛";
/// A heart symbol (♥).
pub const HEART: &str = "♥";
/// An ellipsis symbol (…).
pub const ELLIPSIS: &str = "…";

/// Pre-defined animation frame sets usable with [`crate::spinner::Spinner`].
pub mod frames {
    /// Classic braille-dot spinner frames.
    pub const DOTS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    /// Simple ASCII line spinner frames (safe for limited terminals).
    pub const LINE: &[&str] = &["-", "\\", "|", "/"];
    /// Growing/shrinking vertical bar frames.
    pub const BAR: &[&str] = &["▁", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃"];
    /// Rotating arrow frames.
    pub const ARROW: &[&str] = &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
}