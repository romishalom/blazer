//! A minimal table renderer for displaying tabular data in the terminal.

/// Horizontal alignment for table cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Align content to the left (default).
    Left,
    /// Center content within the column.
    Center,
    /// Align content to the right.
    Right,
}

/// A simple table builder for rendering tabular CLI output.
///
/// # Examples
///
/// ```
/// use blazer::table::Table;
///
/// let mut table = Table::new(vec!["Name", "Age", "City"]);
/// table.add_row(vec!["Alice", "30", "NYC"]);
/// table.add_row(vec!["Bob", "25", "LA"]);
/// println!("{}", table.render());
/// ```
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    alignment: Alignment,
}

impl Table {
    /// Creates a new table with the given column headers.
    pub fn new(headers: Vec<&str>) -> Self {
        Table {
            headers: headers.into_iter().map(String::from).collect(),
            rows: Vec::new(),
            alignment: Alignment::Left,
        }
    }

    /// Sets the alignment used for all cells.
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Adds a row of data to the table.
    ///
    /// Rows with fewer cells than there are headers will have missing
    /// cells rendered as empty strings.
    pub fn add_row(&mut self, row: Vec<&str>) {
        self.rows.push(row.into_iter().map(String::from).collect());
    }

    /// Computes the maximum display width needed for each column.
    fn column_widths(&self) -> Vec<usize> {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.chars().count()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i >= widths.len() {
                    widths.push(cell.chars().count());
                } else {
                    widths[i] = widths[i].max(cell.chars().count());
                }
            }
        }
        widths
    }

    /// Pads `text` to `width` according to the table's alignment setting.
    fn pad(&self, text: &str, width: usize) -> String {
        let len = text.chars().count();
        if len >= width {
            return text.to_string();
        }
        let diff = width - len;
        match self.alignment {
            Alignment::Left => format!("{}{}", text, " ".repeat(diff)),
            Alignment::Right => format!("{}{}", " ".repeat(diff), text),
            Alignment::Center => {
                let left = diff / 2;
                let right = diff - left;
                format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
            }
        }
    }

    /// Renders the table as a formatted, box-drawn string ready to print.
    pub fn render(&self) -> String {
        let widths = self.column_widths();
        let mut out = String::new();

        let render_separator = |out: &mut String| {
            out.push('+');
            for w in &widths {
                out.push_str(&"-".repeat(w + 2));
                out.push('+');
            }
            out.push('\n');
        };

        render_separator(&mut out);

        out.push('|');
        for (i, header) in self.headers.iter().enumerate() {
            out.push(' ');
            out.push_str(&self.pad(header, widths[i]));
            out.push_str(" |");
        }
        out.push('\n');

        render_separator(&mut out);

        for row in &self.rows {
            out.push('|');
            for (i, w) in widths.iter().enumerate() {
                let cell = row.get(i).map(String::as_str).unwrap_or("");
                out.push(' ');
                out.push_str(&self.pad(cell, *w));
                out.push_str(" |");
            }
            out.push('\n');
        }

        render_separator(&mut out);

        out
    }

    /// Renders and prints the table directly to stdout.
    pub fn print(&self) {
        print!("{}", self.render());
    }
}