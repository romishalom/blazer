# blazer

[![Crates.io](https://img.shields.io/crates/v/blazer.svg)](https://crates.io/crates/blazer)
[![Docs.rs](https://docs.rs/blazer/badge.svg)](https://docs.rs/blazer)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)


A blazing-fast, dependency-minimal Rust library for styling, coloring, and
enhancing command-line interfaces.

Basic Example:
```rust
use blazer::Stylize;

println!("{}", "Success!".green().bold());
println!("{}", "Warning!".yellow().underline());
println!("{}", "Error!".white().on_red().bold());
println!("{}", "Custom".rgb(255, 140, 0));
```

---

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Features](#features)
  - [Text Styling](#text-styling)
  - [Progress Bars](#progress-bars)
  - [Spinners](#spinners)
  - [Tables](#tables)
  - [CLI Symbols](#cli-symbols)
  - [Terminal Utilities](#terminal-utilities)
- [Color Support](#color-support)
- [API Documentation](#api-documentation)
- [Examples](#examples)
- [Terminal Compatibility](#terminal-compatibility)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

`blazer` gives you a clean, ergonomic API for every common CLI visual need —
from simple colored output to animated spinners and formatted tables — with
**zero mandatory runtime dependencies** beyond `libc` (Unix only) and the
standard library.

| Concern                  | Solution                              |
|--------------------------|---------------------------------------|
| Colors & styles          | `Stylize` trait (chainable)           |
| Progress feedback        | `ProgressBar` widget                  |
| Indeterminate feedback   | `Spinner` (background thread)         |
| Tabular data             | `Table` with box-drawing              |
| Symbol/icon shorthand    | `symbols` module (✔ ✗ ➜ …)            |
| Terminal introspection   | `terminal` module (size, color check) |

---

## Installation

Add `blazer` to your `Cargo.toml`:

```toml
[dependencies]
blazer = "0.1"
```

Or use the command line:

```bash
cargo add blazer
```

---

## Features

### Text Styling

The [`Stylize`](https://docs.rs/blazer/latest/blazer/trait.Stylize.html) trait
implements on `&str` and `String`, giving you **chainable** methods for
foreground colors, background colors, and text attributes.

```rust
use blazer::Stylize;

// 16 standard ANSI colors + 16 bright variants
println!("{}", "red".red());
println!("{}", "bright blue".bright_blue());

// Text attributes (chain freely)
println!("{}", "bold italic underline".bold().italic().underline());
println!("{}", "dim strikethrough".dim().strikethrough());
println!("{}", "reversed".reverse());

// Background colors
println!("{}", "white on magenta".white().on_magenta());
println!("{}", "black on yellow".black().on_yellow());

// Custom foreground / background
println!("{}", "indigo".rgb(75, 0, 130));
println!("{}", "salmon bg".white().on_rgb(250, 128, 114));

// 256-color palette (0-255)
println!("{}", "orange".fixed(202));
```

Every styled string **automatically resets** at the end, so you never have to
worry about leaking styles into subsequent output.

---

### Progress Bars

[`ProgressBar`](https://docs.rs/blazer/latest/blazer/progress/struct.ProgressBar.html)
renders a customizable bar directly to the terminal, updating in place on each
draw call.

```rust
use blazer::progress::ProgressBar;
use std::{thread, time::Duration};

fn main() {
    let mut bar = ProgressBar::new(100)
        .with_label("Downloading")
        .with_width(50)
        .with_chars('▓', '░');

    for i in 0..=100 {
        bar.set_position(i);
        bar.draw();
        thread::sleep(Duration::from_millis(20));
    }

    bar.finish_with_message("Download complete!");
}
```

**Output:**

```
Downloading [▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓] 100% Download complete!
```

**Builder options:**

| Method                    | Description                             |
|---------------------------|-----------------------------------------|
| `with_width(n)`           | Set bar width in characters             |
| `with_chars(filled, empty)` | Choose fill characters                |
| `with_label("text")`      | Display a label before the bar          |
| `show_percentage(bool)`   | Toggle the trailing percentage          |
| `inc(delta)`              | Increment progress                      |
| `finish()` / `finish_with_message(msg)` | Complete the bar            |

---

### Spinners

[`Spinner`](https://docs.rs/blazer/latest/blazer/spinner/struct.Spinner.html)
animates on a **background thread** so your main thread stays free.

```rust
use blazer::spinner::Spinner;
use blazer::symbols::frames;
use std::{thread, time::Duration};

fn main() {
    let mut spinner = Spinner::new("Fetching data…")
        .with_frames(frames::ARROW)
        .with_interval(Duration::from_millis(100));

    spinner.start();
    thread::sleep(Duration::from_secs(3));
    spinner.stop("✓ Data fetched successfully!");
}
```

**Built-in frame sets** in `blazer::symbols::frames`:

| Constant | Animation                         | Example frames      |
|----------|-----------------------------------|---------------------|
| `DOTS`   | Classic braille dots              | `⠋ ⠙ ⠹ ⠸ ⠼ ⠴ …` |
| `LINE`   | Simple ASCII (portable)           | `- \ | /`           |
| `BAR`    | Growing / shrinking bar           | `▁ ▃ ▄ ▅ ▆ ▇ █`   |
| `ARROW`  | Rotating arrow                    | `← ↖ ↑ ↗ → ↘ ↓ ↙` |

You can also supply your own `&'static [&'static str]`.

---

### Tables

[`Table`](https://docs.rs/blazer/latest/blazer/table/struct.Table.html) renders
customizable, neatly aligned, box-drawn tables with flexible padding and border configurations.

```rust
use blazer::table::{Alignment, Borders, Padding, Table};

fn main() {
    let mut table = Table::new(vec!["Package", "Version", "Status"])
        .with_borders(Borders::ASCII)
        .with_alignment(Alignment::Left);

    table.add_row(vec!["serde", "1.0.188", "✔"]);
    table.add_row(vec!["tokio", "1.32.0", "✔"]);
    table.add_row(vec!["blazer", "0.1.0", "✔"]);

    println!("{table}");
}
```

**Output:**

```
+----------+---------+--------+
| Package  | Version | Status |
+----------+---------+--------+
| serde    | 1.0.188 | ✔      |
| tokio    | 1.32.0  | ✔      |
| blazer   | 0.1.0   | ✔      |
+----------+---------+--------+
```

#### Border Presets (`Borders`)

| Constant         | Preview              | Description                              |
|------------------|----------------------|------------------------------------------|
| `Borders::SINGLE`| `─ │ ┼`             | Standard single-line box drawing         |
| `Borders::DOUBLE`| `═ ║ ╬`             | Heavy double-line box drawing            |
| `Borders::ASCII` | `- \| +`             | Plain ASCII characters for maximum portability |
| `Borders::DASHED`| `╌ ╎ ┼`             | Dashed Unicode border style              |
| `Borders::DOTTED`| `┄ ┊ ┼`             | Dotted Unicode border style              |
| `Borders::EMPTY` | *(None)*             | Completely borderless rendering          |

You can also create fully custom borders or turn off borders with `.borderless()`.

```rust
// Custom border
let custom_borders = Borders::empty().horizontal('-').vertical('|');

// Borderless helper
let table = Table::new(vec!["Key", "Value"]).borderless();
```

#### Padding Options (`Padding`)

Control left, right, and fill character properties on cells:

```rust
// Uniform spacing
let pad = Padding::uniform(2);

// Asymmetric padding
let pad = Padding::new(1, 3, ' ');

// Custom fill character
let pad = Padding::uniform_with_fill(1, '.');
```

---

### CLI Symbols

Ready-to-use Unicode symbols for common CLI meaning, living in
`blazer::symbols`:

```rust
use blazer::symbols::*;
use blazer::Stylize;

println!("{} {}", CHECK.green().bold(), "Build passed");
println!("{} {}", CROSS.red().bold(), "Tests failed");
println!("{} {}", WARNING.yellow().bold(), "Deprecated API");
println!("{} {}", INFO.cyan(), "Tip: run --help");
println!("{} {}", ARROW_RIGHT.magenta(), "Continue? [y/N]");
```

| Constant    | Glyph | Typical use            |
|-------------|-------|------------------------|
| `CHECK`     | `✔`   | Success / passed       |
| `CROSS`     | `✖`   | Failure / error        |
| `WARNING`   | `⚠`   | Warning / caution      |
| `INFO`      | `ℹ`   | Informational note     |
| `ARROW_RIGHT` | `➜` | Prompt / direction     |
| `BULLET`    | `•`   | List item              |
| `STAR`      | `★`   | Favorite / rating      |
| `HOURGLASS` | `⌛`  | Waiting / processing   |
| `HEART`     | `♥`   | Appreciation           |
| `ELLIPSIS`  | `…`   | Truncation             |

---

### Terminal Utilities

```rust
use blazer::terminal;

// Get terminal size (columns, rows)
if let Some((cols, rows)) = terminal::size() {
    println!("Terminal is {} cols × {} rows", cols, rows);
}

// Check color support (respects NO_COLOR and TERM=dumb)
if terminal::supports_color() {
    // render with colors
} else {
    // render plain text
}
```

`size()` uses an `ioctl` call on Unix and `GetConsoleScreenBufferInfo` on
Windows, falling back to the `COLUMNS`/`LINES` environment variables.

---

## Color Support

| Mode            | `Color` variant       | ANSI codes | Notes                       |
|-----------------|-----------------------|------------|------------------------------|
| Standard (16)   | `Black` … `White`     | 30–37 / 40–47 | Most portable            |
| Bright (16)     | `BrightBlack` … `BrightWhite` | 90–97 / 100–107 | Widely supported    |
| 256-color       | `Fixed(0..=255)`      | `38;5;n` / `48;5;n` | Modern terminals     |
| Truecolor (RGB) | `Rgb(r, g, b)`        | `38;2;r;g;b` / `48;2;r;g;b` | xterm-256, iTerm2, Kitty, Alacritty, Windows Terminal |

**Recommendation:** use `terminal::supports_color()` to detect basic support
and feature-detect `COLORTERM=truecolor` (or `truecolor`/`24bit` in `TERM`)
if you need to gate RGB output.

---

## API Documentation

Full API documentation is available on **[docs.rs/blazer](https://docs.rs/blazer)**.

To generate and browse it locally:

```bash
cargo doc --open
```

---

## Examples

Runnable examples live in the `examples/` directory:

```bash
cargo run --example basic_colors   # Color and style showcase
cargo run --example progress_bar   # Animated progress bar
cargo run --example spinner_demo   # Background spinner
cargo run --example table_demo     # Box-drawn table
```

---

## Terminal Compatibility

| Terminal / Environment        | Status          |
|-------------------------------|-----------------|
| Windows Terminal              | ✅ Full support |
| iTerm2 (macOS)                | ✅ Full support |
| Alacritty                     | ✅ Full support |
| Kitty                         | ✅ Full support |
| GNOME Terminal                 | ✅ Full support |
| xterm-256color                | ✅ Full support |
| VS Code integrated terminal   | ✅ Full support |
| cmd.exe / legacy conhost      | ⚠️ Basic colors |
| dumb / NO_COLOR set           | ⚠️ Plain text   |
| CI (GitHub Actions, etc.)     | ⚠️ Basic colors |

When in doubt, call `terminal::supports_color()` and degrade gracefully.

---

## MSRV

The minimum supported Rust version is **1.70**.

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-thing`)
3. Run `cargo fmt`, `cargo clippy`, and `cargo test`
4. Open a pull request

---

## License

Licensed under the [MIT License](LICENSE.txt).