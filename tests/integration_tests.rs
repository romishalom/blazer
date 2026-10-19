use blazer::style::strip_ansi;
use blazer::table::Table;
use blazer::{Color, Stylize};

#[test]
fn red_bold_contains_expected_codes() {
    let s = "Hi".red().bold().to_string();
    assert!(s.contains("\x1b["));
    assert!(s.ends_with("\x1b[0m"));
    assert_eq!(strip_ansi(&s), "Hi");
}

#[test]
fn rgb_color_renders_truecolor_code() {
    let s = "x".rgb(10, 20, 30).to_string();
    assert!(s.contains("38;2;10;20;30"));
}

#[test]
fn background_color_applies() {
    let s = "x".on_green().to_string();
    assert!(s.contains("42"));
}

#[test]
fn strip_ansi_removes_all_escapes() {
    let styled = "plain text".color(Color::Magenta).underline().to_string();
    assert_eq!(strip_ansi(&styled), "plain text");
}

#[test]
fn table_renders_headers_and_rows() {
    let mut table = Table::new(vec!["A", "B"]);
    table.add_row(vec!["1", "2"]);
    let rendered = table.render();
    assert!(rendered.contains("A"));
    assert!(rendered.contains("B"));
    assert!(rendered.contains("1"));
    assert!(rendered.contains("2"));
}