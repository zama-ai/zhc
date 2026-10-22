use std::io::{self, IsTerminal, Write};

/// Compute visible width of a string, ignoring ANSI escape sequences.
fn visible_width(s: &str) -> usize {
    let mut width = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            width += 1;
        }
    }
    width
}

/// Center a string (possibly with ANSI codes) in a field of given width.
fn center_ansi(s: &str, width: usize) -> String {
    let vis = visible_width(s);
    if vis >= width {
        return s.to_string();
    }
    let pad = width - vis;
    let left = pad / 2;
    let right = pad - left;
    format!("{:l$}{}{:r$}", "", s, "", l = left, r = right)
}

/// A table that prints incrementally to TTY, resizing columns dynamically.
/// Falls back to buffered output when stdout is not a terminal.
pub struct DynamicTable {
    columns: Vec<String>,
    rows: Vec<String>,
    cells: Vec<Vec<Option<String>>>,
    col_widths: Vec<usize>,
    row_header_width: usize,
    lines_printed: usize,
    rows_printed: usize,
    is_tty: bool,
    row_separators: bool,
}

impl DynamicTable {
    pub fn new(
        columns: impl IntoIterator<Item = String>,
        rows: impl IntoIterator<Item = String>,
    ) -> Self {
        let columns: Vec<_> = columns.into_iter().collect();
        let rows: Vec<_> = rows.into_iter().collect();
        let col_widths = columns.iter().map(|s| s.len()).collect();
        let row_header_width = rows.iter().map(|s| s.len()).max().unwrap_or(0).max(9) + 2;
        let cells = vec![vec![None; columns.len()]; rows.len()];

        Self {
            columns,
            rows,
            cells,
            col_widths,
            row_header_width,
            lines_printed: 0,
            rows_printed: 0,
            is_tty: io::stdout().is_terminal(),
            row_separators: false,
        }
    }

    /// Enable separators between data rows.
    pub fn with_row_separators(mut self) -> Self {
        self.row_separators = true;
        self
    }

    /// Set a cell value. On TTY, triggers incremental print or full reprint if width changed.
    pub fn set(&mut self, row: usize, col: usize, value: String) {
        let vis_width = value.lines().map(visible_width).max().unwrap_or(0);
        let width_changed = vis_width > self.col_widths[col];
        if width_changed {
            self.col_widths[col] = vis_width;
        }
        self.cells[row][col] = Some(value);

        if self.is_tty {
            if width_changed && self.lines_printed > 0 {
                self.rewind_and_reprint();
            } else if self.should_print_row(row) {
                self.print_row(row);
            }
        }
    }

    fn should_print_row(&self, row: usize) -> bool {
        let row_complete = self.cells[row].iter().all(|c| c.is_some());
        row_complete && row >= self.rows_printed
    }

    fn rewind_and_reprint(&mut self) {
        if self.lines_printed > 0 {
            print!("\x1b[{}A\x1b[J", self.lines_printed);
            io::stdout().flush().unwrap();
        }
        self.lines_printed = 0;
        self.rows_printed = 0;
        self.print_table_so_far();
    }

    fn print_table_so_far(&mut self) {
        self.print_top_border();
        self.print_header();
        self.print_header_separator();
        for i in 0..self.cells.len() {
            if self.cells[i].iter().all(|c| c.is_some()) {
                if self.row_separators && self.rows_printed > 0 {
                    self.print_row_separator();
                }
                self.print_data_row(i);
            }
        }
    }

    fn print_row(&mut self, row: usize) {
        if self.lines_printed == 0 {
            self.print_top_border();
            self.print_header();
            self.print_header_separator();
        } else if self.row_separators && self.rows_printed > 0 {
            self.print_row_separator();
        }
        self.print_data_row(row);
    }

    fn print_top_border(&mut self) {
        print!("┌{:─<w$}", "", w = self.row_header_width);
        for &cw in &self.col_widths {
            print!("┬{:─<w$}", "", w = cw + 2);
        }
        println!("┐");
        self.lines_printed += 1;
    }

    fn print_header(&mut self) {
        print!("│{:^w$}", "Operation", w = self.row_header_width);
        for (col, cw) in self.col_widths.iter().enumerate() {
            print!("│{:^w$}", &self.columns[col], w = cw + 2);
        }
        println!("│");
        self.lines_printed += 1;
    }

    fn print_header_separator(&mut self) {
        print!("├{:─<w$}", "", w = self.row_header_width);
        for &cw in &self.col_widths {
            print!("┼{:─<w$}", "", w = cw + 2);
        }
        println!("┤");
        self.lines_printed += 1;
    }

    fn print_row_separator(&mut self) {
        print!("├{:─<w$}", "", w = self.row_header_width);
        for &cw in &self.col_widths {
            print!("┼{:─<w$}", "", w = cw + 2);
        }
        println!("┤");
        self.lines_printed += 1;
    }

    fn print_data_row(&mut self, row: usize) {
        let cell_lines: Vec<Vec<&str>> = self.cells[row]
            .iter()
            .map(|c| c.as_deref().unwrap_or("-").lines().collect())
            .collect();

        let height = cell_lines.iter().map(|c| c.len()).max().unwrap_or(1);

        for line_idx in 0..height {
            let row_header = if line_idx == 0 { &self.rows[row] } else { "" };
            print!("│ {:<w$}", row_header, w = self.row_header_width - 1);
            for (col, cw) in self.col_widths.iter().enumerate() {
                let line = cell_lines[col].get(line_idx).copied().unwrap_or("");
                print!("│{}", center_ansi(line, cw + 2));
            }
            println!("│");
            self.lines_printed += 1;
        }
        io::stdout().flush().unwrap();
        self.rows_printed += 1;
    }

    fn print_bottom_border(&mut self) {
        print!("└{:─<w$}", "", w = self.row_header_width);
        for &cw in &self.col_widths {
            print!("┴{:─<w$}", "", w = cw + 2);
        }
        println!("┘");
    }

    /// Finalize the table, printing bottom border (or entire table if not TTY).
    pub fn finish(mut self) {
        if !self.is_tty {
            self.print_table_so_far();
        }
        self.print_bottom_border();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_basic_usage() {
        // Create a table with column headers (bit widths) and row headers (operations)
        let columns = ["8b", "16b", "32b"].map(String::from);
        let rows = ["Add", "Mul", "Sub"].map(String::from);

        let mut table = DynamicTable::new(columns, rows);

        // Fill cells row by row (typical bench iteration order)
        for row in 0..3 {
            for col in 0..3 {
                let value = format!("{} µs", (row + 1) * (col + 1) * 100);
                table.set(row, col, value);
            }
        }

        table.finish();
    }

    #[test]
    fn api_with_dynamic_width() {
        // Demonstrates column resizing when a wider value appears
        let columns = ["8b", "16b"].map(String::from);
        let rows = ["Add", "Mul"].map(String::from);

        let mut table = DynamicTable::new(columns, rows);

        // First row: short values
        table.set(0, 0, "42".into());
        table.set(0, 1, "58".into());

        // Second row: one very wide value triggers reprint
        table.set(1, 0, "1 234 567 µs".into());
        table.set(1, 1, "99".into());

        table.finish();
    }

    #[test]
    fn api_analyze_use_case() {
        // Simulates the analyze command use case
        let bits = [8, 16, 32];
        let ops = ["Add", "Mul", "CmpLt"];

        let columns = bits.iter().map(|b| format!("{}b", b));
        let rows = ops.iter().map(|s| s.to_string());

        let mut table = DynamicTable::new(columns, rows);

        // Simulate computing analysis for each (op, bits) pair
        for (row_idx, op) in ops.iter().enumerate() {
            for (col_idx, bits) in bits.iter().enumerate() {
                // This would be: let ir = builder.ir(); analyze_ir(&ir)
                let analysis_result = format!("{} ops @ {}b", op.len() * 10, bits);
                table.set(row_idx, col_idx, analysis_result);
            }
        }

        table.finish();
    }

    #[test]
    fn api_partial_fill() {
        // Not all cells need to be filled
        let columns = ["A", "B", "C"].map(String::from);
        let rows = ["X", "Y"].map(String::from);

        let mut table = DynamicTable::new(columns, rows);

        // Only fill first row completely
        table.set(0, 0, "1".into());
        table.set(0, 1, "2".into());
        table.set(0, 2, "3".into());

        // Second row partial - missing cells show as "-"
        table.set(1, 1, "only middle".into());

        table.finish();
    }

    #[test]
    fn api_multiline_cells() {
        let columns = ["8b", "16b"].map(String::from);
        let rows = ["Add", "Mul"].map(String::from);

        let mut table = DynamicTable::new(columns, rows).with_row_separators();

        table.set(0, 0, "line1\nline2\nline3".into());
        table.set(0, 1, "short".into());
        table.set(1, 0, "single".into());
        table.set(1, 1, "also\nmulti".into());

        table.finish();
    }

    #[test]
    #[ignore] // Run with: cargo test -p zhc_utils table_live_demo -- --ignored --nocapture
    fn table_live_demo() {
        use std::thread::sleep;
        use std::time::Duration;

        let columns = ["8b", "16b", "32b"].map(String::from);
        let rows = ["Add", "Mul", "Div", "CmpLt"].map(String::from);

        let mut table = DynamicTable::new(columns, rows);

        // Row 0: short values
        for col in 0..3 {
            sleep(Duration::from_millis(300));
            table.set(0, col, format!("{}", 10 + col));
        }

        // Row 1: medium values
        for col in 0..3 {
            sleep(Duration::from_millis(300));
            table.set(1, col, format!("{} µs", 100 * (col + 1)));
        }

        // Row 2: one wide value triggers reprint
        sleep(Duration::from_millis(300));
        table.set(2, 0, "42".into());
        sleep(Duration::from_millis(300));
        table.set(2, 1, "1 234 567 µs".into()); // <-- column grows, full reprint
        sleep(Duration::from_millis(300));
        table.set(2, 2, "99".into());

        // Row 3: back to normal
        for col in 0..3 {
            sleep(Duration::from_millis(300));
            table.set(3, col, format!("done {}", col));
        }

        sleep(Duration::from_millis(500));
        table.finish();
    }
}
