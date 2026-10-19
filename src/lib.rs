//! # Blazer
//!
//! `blazer` is a lightweight, dependency-minimal library for building
//! beautiful command-line interfaces in Rust. It provides:
//!
//! - **Text styling & coloring** via the [`Stylize`] trait — chainable,
//!   ergonomic methods like `.red()`, `.bold()`, `.on_blue()`.
//! - **True color & 256-color support** via [`Color::Rgb`] and [`Color::Fixed`].
//! - **Progress bars** via [`progress::ProgressBar`].
//! - **Animated spinners** via [`spinner::Spinner`].
//! - **Tables** via [`table::Table`].
//! - **Common CLI symbols** (checkmarks, crosses, arrows, etc.) via the
//!   [`symbols`] module.
//! - **Terminal utilities** (size detection, color support detection) via
//!   the [`terminal`] module.
//!
//! ## Quick start
//!
//! ```
//! use blazer::Stylize;
//!
//! println!("{}", "Success!".green().bold());
//! println!("{}", "Warning".yellow().italic());
//! println!("{}", "Error".white().on_red().bold());
//! println!("{}", "Custom".rgb(255, 140, 0));
//! ```
//!
//! ## Progress bars
//!
//! ```no_run
//! use blazer::progress::ProgressBar;
//!
//! let mut bar = ProgressBar::new(50).with_label("Downloading");
//! for i in 0..=50 {
//!     bar.set_position(i);
//!     bar.draw();
//!     std::thread::sleep(std::time::Duration::from_millis(20));
//! }
//! bar.finish_with_message("Done!");
//! ```
//!
//! ## Spinners
//!
//! ```no_run
//! use blazer::spinner::Spinner;
//!
//! let mut spinner = Spinner::new("Working...");
//! spinner.start();
//! std::thread::sleep(std::time::Duration::from_secs(2));
//! spinner.stop("Finished!");
//! ```
//!
//! ## Tables
//!
//! ```
//! use blazer::table::Table;
//!
//! let mut table = Table::new(vec!["Name", "Status"]);
//! table.add_row(vec!["build", "ok"]);
//! table.add_row(vec!["test", "failed"]);
//! table.print();
//! ```
//!
//! ## Module overview
//!
//! | Module      | Purpose                                     |
//! |-------------|----------------------------------------------|
//! | [`color`]   | Color enums and ANSI code generation          |
//! | [`style`]   | The [`Stylize`] trait and [`StyledString`]    |
//! | [`progress`]| Progress bar widget                           |
//! | [`spinner`] | Animated spinner widget                       |
//! | [`table`]   | Simple table renderer                         |
//! | [`symbols`] | Common CLI Unicode symbols                    |
//! | [`terminal`]| Terminal size & color support detection       |

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod color;
pub mod progress;
pub mod spinner;
pub mod style;
pub mod symbols;
pub mod table;
pub mod terminal;

pub use color::Color;
pub use style::{strip_ansi, StyledString, Stylize};