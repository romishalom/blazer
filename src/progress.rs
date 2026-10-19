//! A simple, customizable terminal progress bar.

use std::io::{self, Write};

/// A customizable progress bar for displaying task completion in the terminal.
///
/// # Examples
///
/// ```no_run
/// use blazer::progress::ProgressBar;
///
/// let mut bar = ProgressBar::new(100);
/// for i in 0..=100 {
///     bar.set_position(i);
///     bar.draw();
///     std::thread::sleep(std::time::Duration::from_millis(10));
/// }
/// bar.finish();
/// ```
pub struct ProgressBar {
    total: u64,
    position: u64,
    width: usize,
    filled_char: char,
    empty_char: char,
    label: Option<String>,
    show_percentage: bool,
}

impl ProgressBar {
    /// Creates a new progress bar with the given total number of steps.
    pub fn new(total: u64) -> Self {
        ProgressBar {
            total: total.max(1),
            position: 0,
            width: 40,
            filled_char: '█',
            empty_char: '░',
            label: None,
            show_percentage: true,
        }
    }

    /// Sets the visual width (in characters) of the bar itself.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets the characters used to render filled and empty segments.
    pub fn with_chars(mut self, filled: char, empty: char) -> Self {
        self.filled_char = filled;
        self.empty_char = empty;
        self
    }

    /// Sets a text label to display alongside the bar.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Enables or disables the percentage display.
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Sets the current progress position (clamped to `[0, total]`).
    pub fn set_position(&mut self, position: u64) {
        self.position = position.min(self.total);
    }

    /// Increments the progress position by `delta` steps.
    pub fn inc(&mut self, delta: u64) {
        self.set_position(self.position + delta);
    }

    /// Returns the current progress ratio, between `0.0` and `1.0`.
    pub fn ratio(&self) -> f64 {
        self.position as f64 / self.total as f64
    }

    /// Renders the current state of the progress bar to stdout, overwriting
    /// the previous line.
    pub fn draw(&self) {
        let ratio = self.ratio();
        let filled_len = ((self.width as f64) * ratio).round() as usize;
        let filled_len = filled_len.min(self.width);

        let mut line = String::new();
        line.push('\r');

        if let Some(label) = &self.label {
            line.push_str(label);
            line.push(' ');
        }

        line.push('[');
        for _ in 0..filled_len {
            line.push(self.filled_char);
        }
        for _ in filled_len..self.width {
            line.push(self.empty_char);
        }
        line.push(']');

        if self.show_percentage {
            line.push_str(&format!(" {:>3.0}%", ratio * 100.0));
        }

        print!("{}", line);
        let _ = io::stdout().flush();
    }

    /// Completes the progress bar, setting it to 100% and moving to a new line.
    pub fn finish(&mut self) {
        self.position = self.total;
        self.draw();
        println!();
    }

    /// Completes the progress bar with a custom final message.
    pub fn finish_with_message(&mut self, message: impl AsRef<str>) {
        self.position = self.total;
        self.draw();
        println!(" {}", message.as_ref());
    }
}