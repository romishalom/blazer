//! A minimal table renderer for displaying tabular data in the terminal.

/// Padding configuration for table cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    left: usize,
    right: usize,
    fill: char,
}

impl Padding {
    /// Create a padding configuration with explicit values for all sides
    pub const fn new(left: usize, right: usize, fill: char) -> Self {
        Self { left, right, fill }
    }

    /// Helper for even padding on all four sides
    pub const fn uniform(size: usize) -> Self {
        Self { left: size, right: size, fill: ' ' }
    }

    /// Helper for even padding on all four sides with a custom fill character
    pub const fn uniform_with_fill(size: usize, fill: char) -> Self {
        Self { left: size, right: size, fill }
    }

    /// Sets the padding on the left side
    pub const fn with_left(mut self, left: usize) -> Self {
        self.left = left;
        self
    }

    /// Sets the padding on the right side
    pub const fn with_right(mut self, right: usize) -> Self {
        self.right = right;
        self
    }

    /// Sets the padding fill character
    pub const fn with_fill(mut self, fill: char) -> Self {
        self.fill = fill;
        self
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self { left: 1, right: 1, fill: ' ', }
    }
}

/// Border configuration for table cells.
pub struct Borders {
    horizontal: Option<char>,
    vertical: Option<char>,
    corner: Option<char>,
}

impl Borders {

    /// Creates a border layout with the given characters.
    pub fn new(horizontal: impl Into<Option<char>>, vertical: impl Into<Option<char>>, corner: impl Into<Option<char>>) -> Self {
        Self { horizontal: horizontal.into(), vertical: vertical.into(), corner: corner.into() }
    }

    /// Blank border layout where all fields are `None`.
    pub const EMPTY: Self = Self { horizontal: None, vertical: None, corner: None, };

    /// Standard single-line Unicode border style.
    pub const SINGLE: Self = Self { horizontal: Some('─'), vertical: Some('│'), corner: Some('┼'), };

    /// Heavy double-line Unicode border style.
    pub const DOUBLE: Self = Self { horizontal: Some('═'), vertical: Some('║'), corner: Some('╬'), };

    /// Dashed Unicode border style.
    pub const DASHED: Self = Self { horizontal: Some('╌'), vertical: Some('╎'), corner: Some('┼'), };

    /// Dotted Unicode border style.
    pub const DOTTED: Self = Self { horizontal: Some('┄'), vertical: Some('┊'), corner: Some('┼'), };

    /// Pure ASCII border style.
    pub const ASCII: Self = Self { horizontal: Some('-'), vertical: Some('|'), corner: Some('+'), };

    /// Sets the horizontal border character.
    pub fn horizontal(mut self, c: impl Into<Option<char>>) -> Self {
        self.horizontal = c.into();
        self
    }

    /// Sets the vertical border character.
    pub fn vertical(mut self, c: impl Into<Option<char>>) -> Self {
        self.vertical = c.into();
        self
    }

    /// Sets the corner
    pub fn corner(mut self, c: impl Into<Option<char>>) -> Self {
        self.corner = c.into();
        self
    }
}

impl Default for Borders {
    fn default() -> Self {
        Self::SINGLE
    }
}

/// Horizontal alignment for table cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Align content to the left (default).
    #[default]
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
    padding: Padding,
    borders: Borders,
}

impl Table {
    /// Creates a new table with the given column headers.
    pub fn new(headers: Vec<&str>) -> Self {
        Table {
            headers: headers.into_iter().map(String::from).collect(),
            rows: Vec::new(),
            alignment: Alignment::Left,
            padding: Padding::default(),
            borders: Borders::ASCII,
        }
    }

    /// Sets the alignment used for all cells.
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Sets the border characters used for the table.
    pub fn with_borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    /// Disables all borders.
    pub fn borderless(mut self) -> Self {
        self.borders = Borders::EMPTY;
        self
    }

    /// Sets the padding used for all cells.
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
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
        let fill = self.padding.fill.to_string();
        match self.alignment {
            Alignment::Left => format!("{}{}", text, fill.repeat(diff)),
            Alignment::Right => format!("{}{}", fill.repeat(diff), text),
            Alignment::Center => {
                let left = diff / 2;
                let right = diff - left;
                format!("{}{}{}", fill.repeat(left), text, fill.repeat(right))
            }
        }
    }

    /// Renders the table as a formatted, box-drawn string ready to print.
    pub fn render(&self) -> String {
        let widths = self.column_widths();
        let mut out = String::new();

        let render_separator = |out: &mut String, | {
            if let Some(corner) = self.borders.corner {
                out.push(corner);
            }
            for w in &widths {
                if let Some(horizontal_border) = self.borders.horizontal {
                    out.push_str(&horizontal_border.to_string().repeat(w + self.padding.left + self.padding.right));
                }
                if let Some(corner) = self.borders.corner {
                    out.push(corner);
                }
            }
            out.push('\n');
        };

        render_separator(&mut out);

        if let Some(vertical_border) = self.borders.vertical {
            out.push(vertical_border);
        }
        for (i, header) in self.headers.iter().enumerate() {
            out.push_str(&self.padding.fill.to_string().repeat(self.padding.left));
            out.push_str(&self.pad(header, widths[i]));
            out.push_str(&self.padding.fill.to_string().repeat(self.padding.right));
            if let Some(vertical_border) = self.borders.vertical {
                out.push(vertical_border);
            }
        }
        out.push('\n');

        render_separator(&mut out);

        for row in &self.rows {
            if let Some(vertical_border) = self.borders.vertical {
                out.push(vertical_border);
            }
            for (i, w) in widths.iter().enumerate() {
                let cell = row.get(i).map(String::as_str).unwrap_or("");

                out.push_str(&self.padding.fill.to_string().repeat(self.padding.left));
                out.push_str(&self.pad(cell, *w));
                out.push_str(&self.padding.fill.to_string().repeat(self.padding.right));
                if let Some(vertical_border) = self.borders.vertical {
                    out.push(vertical_border);
                }
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

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}