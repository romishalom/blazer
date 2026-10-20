//! Color definitions and utilities for terminal output.
//!
//! This module provides the [`Color`] enum, which represents colors
//! that can be applied to terminal text — standard 16 ANSI colors,
//! 256-color palette entries, or true 24-bit RGB colors.

use std::fmt;

/// Represents a color that can be applied to terminal text.
///
/// Blazer supports three color modes:
/// - **Standard**: The classic 16 ANSI colors (8 normal + 8 bright)
/// - **Fixed**: 256-color palette (0-255)
/// - **Rgb**: True color (24-bit RGB)
///
/// # Examples
///
/// ```
/// use blazer::Color;
///
/// let red = Color::Red;
/// let custom = Color::Rgb(255, 128, 0);
/// let palette = Color::Fixed(202);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Standard black.
    Black,
    /// Standard red.
    Red,
    /// Standard green.
    Green,
    /// Standard yellow.
    Yellow,
    /// Standard blue.
    Blue,
    /// Standard magenta.
    Magenta,
    /// Standard cyan.
    Cyan,
    /// Standard white.
    White,
    /// Bright black (gray).
    BrightBlack,
    /// Bright red.
    BrightRed,
    /// Bright green.
    BrightGreen,
    /// Bright yellow.
    BrightYellow,
    /// Bright blue.
    BrightBlue,
    /// Bright magenta.
    BrightMagenta,
    /// Bright cyan.
    BrightCyan,
    /// Bright white.
    BrightWhite,
    /// A color from the 256-color palette (0-255).
    Fixed(u8),
    /// A true-color (24-bit) RGB value.
    Rgb(u8, u8, u8),
}

impl Color {
    /// Returns the ANSI foreground escape code fragment for this color
    /// (without the leading `\x1b[` or trailing `m`).
    pub(crate) fn fg_code(&self) -> String {
        match self {
            Color::Black => "30".to_string(),
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
            Color::BrightBlack => "90".to_string(),
            Color::BrightRed => "91".to_string(),
            Color::BrightGreen => "92".to_string(),
            Color::BrightYellow => "93".to_string(),
            Color::BrightBlue => "94".to_string(),
            Color::BrightMagenta => "95".to_string(),
            Color::BrightCyan => "96".to_string(),
            Color::BrightWhite => "97".to_string(),
            Color::Fixed(n) => format!("38;5;{}", n),
            Color::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }

    /// Returns the ANSI background escape code fragment for this color
    /// (without the leading `\x1b[` or trailing `m`).
    pub(crate) fn bg_code(&self) -> String {
        match self {
            Color::Black => "40".to_string(),
            Color::Red => "41".to_string(),
            Color::Green => "42".to_string(),
            Color::Yellow => "43".to_string(),
            Color::Blue => "44".to_string(),
            Color::Magenta => "45".to_string(),
            Color::Cyan => "46".to_string(),
            Color::White => "47".to_string(),
            Color::BrightBlack => "100".to_string(),
            Color::BrightRed => "101".to_string(),
            Color::BrightGreen => "102".to_string(),
            Color::BrightYellow => "103".to_string(),
            Color::BrightBlue => "104".to_string(),
            Color::BrightMagenta => "105".to_string(),
            Color::BrightCyan => "106".to_string(),
            Color::BrightWhite => "107".to_string(),
            Color::Fixed(n) => format!("48;5;{}", n),
            Color::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        }
    }
}

impl fmt::Display for Color {
    /// Writes the raw ANSI foreground escape sequence for this color.
    ///
    /// Note: this does **not** emit a reset code. Prefer the [`crate::Stylize`]
    /// trait for safe, self-resetting styled output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}m", self.fg_code())
    }
}