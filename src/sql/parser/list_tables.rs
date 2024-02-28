use crate::sql::RUSQLError;
use crate::Database;
use colored::*;
use prettytable as tb;
use prettytable::{format, Cell, Row};

pub fn list_tables(db: &Database) -> Result<String, RUSQLError> {
    let mut table = tb::Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["S.No", "Table Name"]);

    for (i, table_name) in db.tables.keys().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&(i + 1).to_string()).style_spec("Fb"),
            Cell::new(table_name).style_spec("Fb"),
        ]));
    }
    table.printstd();

    Ok(String::from("LIST TABLES Statement executed.")
        .green()
        .to_string())
}
