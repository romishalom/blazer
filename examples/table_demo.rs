use blazer::table::{Alignment, Borders, Table};

fn main() {
    let mut table = Table::new(vec!["Task", "Status", "Duration"])
        .with_borders(Borders::ASCII)
        .with_alignment(Alignment::Center);
    table.add_row(vec!["build", "ok", "4.2s"]);
    table.add_row(vec!["test", "failed", "12.9s"]);
    table.add_row(vec!["lint", "ok", "1.1s"]);


    table.print();
}