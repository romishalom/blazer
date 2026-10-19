//! Text styling functionality including colors, bold, italic, underline, and more.
//!
//! This is the core module of Blazer, providing the [`Stylize`] trait that
//! extends string types with chainable styling methods, and the
//! [`StyledString`] type that holds the resulting style information.

use crate::color::Color;
use std::fmt;

const RESET: &str = "\x1b[0m";

/// Internal set of boolean text attributes (bold, italic, etc.).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Attributes {
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    reverse: bool,
    hidden: bool,
    strikethrough: bool,
}

/// A string value paired with styling information (color, boldness, etc.).
///
/// `StyledString` is produced by calling methods from the [`Stylize`] trait
/// on any `&str` or `String`, or by constructing one directly with
/// [`StyledString::new`]. It implements [`Display`](fmt::Display), so it can
/// be printed directly or formatted into other strings.
///
/// # Examples
///
/// ```
/// use blazer::Stylize;
///
/// let s = "Hello".red().bold();
/// println!("{}", s);
/// assert_eq!(s.to_string(), "\u{1b}[1;31mHello\u{1b}[0m");
/// ```
#[derive(Debug, Clone)]
pub struct StyledString {
    text: String,
    fg: Option<Color>,
    bg: Option<Color>,
    attrs: Attributes,
}

// Generates a chainable foreground color method on `StyledString`.
macro_rules! inherent_color_method {
    ($name:ident, $doc:expr, $color:expr) => {
        #[doc = $doc]
        pub fn $name(self) -> StyledString {
            self.color($color)
        }
    };
}

// Generates a chainable background color method on `StyledString`.
macro_rules! inherent_bg_method {
    ($name:ident, $doc:expr, $color:expr) => {
        #[doc = $doc]
        pub fn $name(self) -> StyledString {
            self.on_color($color)
        }
    };
}

impl StyledString {
    /// Creates a new, unstyled `StyledString` from any value convertible
    /// into a `String`.
    pub fn new(text: impl Into<String>) -> Self {
        StyledString {
            text: text.into(),
            fg: None,
            bg: None,
            attrs: Attributes::default(),
        }
    }

    /// Sets the foreground color.
    pub fn color(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color.
    pub fn on_color(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Makes the text bold.
    pub fn bold(mut self) -> Self {
        self.attrs.bold = true;
        self
    }

    /// Makes the text dim/faint.
    pub fn dim(mut self) -> Self {
        self.attrs.dim = true;
        self
    }

    /// Makes the text italic.
    pub fn italic(mut self) -> Self {
        self.attrs.italic = true;
        self
    }

    /// Underlines the text.
    pub fn underline(mut self) -> Self {
        self.attrs.underline = true;
        self
    }

    /// Makes the text blink (not supported by all terminals).
    pub fn blink(mut self) -> Self {
        self.attrs.blink = true;
        self
    }

    /// Reverses the foreground and background colors.
    pub fn reverse(mut self) -> Self {
        self.attrs.reverse = true;
        self
    }

    /// Hides the text (useful for secrets); matches foreground to background.
    pub fn hidden(mut self) -> Self {
        self.attrs.hidden = true;
        self
    }

    /// Adds a strikethrough to the text.
    pub fn strikethrough(mut self) -> Self {
        self.attrs.strikethrough = true;
        self
    }

    /// Sets a 256-color palette foreground color.
    pub fn fixed(self, n: u8) -> Self {
        self.color(Color::Fixed(n))
    }

    /// Sets an RGB truecolor foreground color.
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.color(Color::Rgb(r, g, b))
    }

    /// Sets an RGB truecolor background color.
    pub fn on_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.on_color(Color::Rgb(r, g, b))
    }

    inherent_color_method!(black, "Sets foreground to black.", Color::Black);
    inherent_color_method!(red, "Sets foreground to red.", Color::Red);
    inherent_color_method!(green, "Sets foreground to green.", Color::Green);
    inherent_color_method!(yellow, "Sets foreground to yellow.", Color::Yellow);
    inherent_color_method!(blue, "Sets foreground to blue.", Color::Blue);
    inherent_color_method!(magenta, "Sets foreground to magenta.", Color::Magenta);
    inherent_color_method!(cyan, "Sets foreground to cyan.", Color::Cyan);
    inherent_color_method!(white, "Sets foreground to white.", Color::White);
    inherent_color_method!(bright_black, "Sets foreground to bright black.", Color::BrightBlack);
    inherent_color_method!(bright_red, "Sets foreground to bright red.", Color::BrightRed);
    inherent_color_method!(bright_green, "Sets foreground to bright green.", Color::BrightGreen);
    inherent_color_method!(bright_yellow, "Sets foreground to bright yellow.", Color::BrightYellow);
    inherent_color_method!(bright_blue, "Sets foreground to bright blue.", Color::BrightBlue);
    inherent_color_method!(bright_magenta, "Sets foreground to bright magenta.", Color::BrightMagenta);
    inherent_color_method!(bright_cyan, "Sets foreground to bright cyan.", Color::BrightCyan);
    inherent_color_method!(bright_white, "Sets foreground to bright white.", Color::BrightWhite);

    inherent_bg_method!(on_black, "Sets background to black.", Color::Black);
    inherent_bg_method!(on_red, "Sets background to red.", Color::Red);
    inherent_bg_method!(on_green, "Sets background to green.", Color::Green);
    inherent_bg_method!(on_yellow, "Sets background to yellow.", Color::Yellow);
    inherent_bg_method!(on_blue, "Sets background to blue.", Color::Blue);
    inherent_bg_method!(on_magenta, "Sets background to magenta.", Color::Magenta);
    inherent_bg_method!(on_cyan, "Sets background to cyan.", Color::Cyan);
    inherent_bg_method!(on_white, "Sets background to white.", Color::White);
    inherent_bg_method!(on_bright_black, "Sets background to bright black.", Color::BrightBlack);
    inherent_bg_method!(on_bright_red, "Sets background to bright red.", Color::BrightRed);
    inherent_bg_method!(on_bright_green, "Sets background to bright green.", Color::BrightGreen);
    inherent_bg_method!(on_bright_yellow, "Sets background to bright yellow.", Color::BrightYellow);
    inherent_bg_method!(on_bright_blue, "Sets background to bright blue.", Color::BrightBlue);
    inherent_bg_method!(on_bright_magenta, "Sets background to bright magenta.", Color::BrightMagenta);
    inherent_bg_method!(on_bright_cyan, "Sets background to bright cyan.", Color::BrightCyan);
    inherent_bg_method!(on_bright_white, "Sets background to bright white.", Color::BrightWhite);

    /// Builds the ANSI escape prefix representing all currently-set styles.
    fn build_prefix(&self) -> String {
        let mut codes: Vec<String> = Vec::new();

        if self.attrs.bold {
            codes.push("1".into());
        }
        if self.attrs.dim {
            codes.push("2".into());
        }
        if self.attrs.italic {
            codes.push("3".into());
        }
        if self.attrs.underline {
            codes.push("4".into());
        }
        if self.attrs.blink {
            codes.push("5".into());
        }
        if self.attrs.reverse {
            codes.push("7".into());
        }
        if self.attrs.hidden {
            codes.push("8".into());
        }
        if self.attrs.strikethrough {
            codes.push("9".into());
        }
        if let Some(fg) = self.fg {
            codes.push(fg.fg_code());
        }
        if let Some(bg) = self.bg {
            codes.push(bg.bg_code());
        }

        if codes.is_empty() {
            String::new()
        } else {
            format!("\x1b[{}m", codes.join(";"))
        }
    }
}

impl fmt::Display for StyledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = self.build_prefix();
        if prefix.is_empty() {
            write!(f, "{}", self.text)
        } else {
            write!(f, "{}{}{}", prefix, self.text, RESET)
        }
    }
}

// Generates a chainable foreground color default-method on the `Stylize` trait.
macro_rules! trait_color_method {
    ($name:ident, $doc:expr, $color:expr) => {
        #[doc = $doc]
        fn $name(self) -> StyledString {
            self.stylize().color($color)
        }
    };
}

// Generates a chainable background color default-method on the `Stylize` trait.
macro_rules! trait_bg_method {
    ($name:ident, $doc:expr, $color:expr) => {
        #[doc = $doc]
        fn $name(self) -> StyledString {
            self.stylize().on_color($color)
        }
    };
}

/// Extension trait providing chainable styling methods for string-like types.
///
/// Implemented for `&str` and `String`, allowing you to write expressive,
/// chainable color and style declarations without manually constructing
/// ANSI escape codes.
///
/// # Examples
///
/// ```
/// use blazer::Stylize;
///
/// println!("{}", "Error!".red().bold());
/// println!("{}", "Success".green().underline());
/// println!("{}", "Custom".rgb(255, 100, 0));
/// println!("{}", "Highlighted".black().on_yellow());
/// ```
pub trait Stylize: Sized {
    /// Converts `self` into a [`StyledString`] with no styling applied yet.
    fn stylize(self) -> StyledString;

    /// Sets an arbitrary foreground [`Color`].
    fn color(self, color: Color) -> StyledString {
        self.stylize().color(color)
    }

    /// Sets an arbitrary background [`Color`].
    fn on_color(self, color: Color) -> StyledString {
        self.stylize().on_color(color)
    }

    /// Makes the text bold.
    fn bold(self) -> StyledString {
        self.stylize().bold()
    }

    /// Makes the text dim/faint.
    fn dim(self) -> StyledString {
        self.stylize().dim()
    }

    /// Makes the text italic.
    fn italic(self) -> StyledString {
        self.stylize().italic()
    }

    /// Underlines the text.
    fn underline(self) -> StyledString {
        self.stylize().underline()
    }

    /// Makes the text blink (not supported by all terminals).
    fn blink(self) -> StyledString {
        self.stylize().blink()
    }

    /// Reverses the foreground and background colors.
    fn reverse(self) -> StyledString {
        self.stylize().reverse()
    }

    /// Hides the text (useful for secrets).
    fn hidden(self) -> StyledString {
        self.stylize().hidden()
    }

    /// Adds a strikethrough to the text.
    fn strikethrough(self) -> StyledString {
        self.stylize().strikethrough()
    }

    /// Sets a 256-color palette foreground color.
    fn fixed(self, n: u8) -> StyledString {
        self.stylize().color(Color::Fixed(n))
    }

    /// Sets an RGB truecolor foreground color.
    fn rgb(self, r: u8, g: u8, b: u8) -> StyledString {
        self.stylize().color(Color::Rgb(r, g, b))
    }

    /// Sets an RGB truecolor background color.
    fn on_rgb(self, r: u8, g: u8, b: u8) -> StyledString {
        self.stylize().on_color(Color::Rgb(r, g, b))
    }

    trait_color_method!(black, "Sets foreground to black.", Color::Black);
    trait_color_method!(red, "Sets foreground to red.", Color::Red);
    trait_color_method!(green, "Sets foreground to green.", Color::Green);
    trait_color_method!(yellow, "Sets foreground to yellow.", Color::Yellow);
    trait_color_method!(blue, "Sets foreground to blue.", Color::Blue);
    trait_color_method!(magenta, "Sets foreground to magenta.", Color::Magenta);
    trait_color_method!(cyan, "Sets foreground to cyan.", Color::Cyan);
    trait_color_method!(white, "Sets foreground to white.", Color::White);
    trait_color_method!(bright_black, "Sets foreground to bright black.", Color::BrightBlack);
    trait_color_method!(bright_red, "Sets foreground to bright red.", Color::BrightRed);
    trait_color_method!(bright_green, "Sets foreground to bright green.", Color::BrightGreen);
    trait_color_method!(bright_yellow, "Sets foreground to bright yellow.", Color::BrightYellow);
    trait_color_method!(bright_blue, "Sets foreground to bright blue.", Color::BrightBlue);
    trait_color_method!(bright_magenta, "Sets foreground to bright magenta.", Color::BrightMagenta);
    trait_color_method!(bright_cyan, "Sets foreground to bright cyan.", Color::BrightCyan);
    trait_color_method!(bright_white, "Sets foreground to bright white.", Color::BrightWhite);

    trait_bg_method!(on_black, "Sets background to black.", Color::Black);
    trait_bg_method!(on_red, "Sets background to red.", Color::Red);
    trait_bg_method!(on_green, "Sets background to green.", Color::Green);
    trait_bg_method!(on_yellow, "Sets background to yellow.", Color::Yellow);
    trait_bg_method!(on_blue, "Sets background to blue.", Color::Blue);
    trait_bg_method!(on_magenta, "Sets background to magenta.", Color::Magenta);
    trait_bg_method!(on_cyan, "Sets background to cyan.", Color::Cyan);
    trait_bg_method!(on_white, "Sets background to white.", Color::White);
    trait_bg_method!(on_bright_black, "Sets background to bright black.", Color::BrightBlack);
    trait_bg_method!(on_bright_red, "Sets background to bright red.", Color::BrightRed);
    trait_bg_method!(on_bright_green, "Sets background to bright green.", Color::BrightGreen);
    trait_bg_method!(on_bright_yellow, "Sets background to bright yellow.", Color::BrightYellow);
    trait_bg_method!(on_bright_blue, "Sets background to bright blue.", Color::BrightBlue);
    trait_bg_method!(on_bright_magenta, "Sets background to bright magenta.", Color::BrightMagenta);
    trait_bg_method!(on_bright_cyan, "Sets background to bright cyan.", Color::BrightCyan);
    trait_bg_method!(on_bright_white, "Sets background to bright white.", Color::BrightWhite);
}

impl Stylize for &str {
    fn stylize(self) -> StyledString {
        StyledString::new(self)
    }
}

impl Stylize for String {
    fn stylize(self) -> StyledString {
        StyledString::new(self)
    }
}

/// Removes ANSI escape sequences from a string, returning the plain text.
///
/// This is useful when you need to calculate the visible width of a string
/// that may contain color or style codes (e.g. for aligning columns).
///
/// # Examples
///
/// ```
/// use blazer::style::strip_ansi;
/// use blazer::Stylize;
///
/// let styled = "Hello".red().to_string();
/// assert_eq!(strip_ansi(&styled), "Hello");
/// ```
pub fn strip_ansi(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if let Some('[') = chars.peek() {
                chars.next();
                for next in chars.by_ref() {
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}