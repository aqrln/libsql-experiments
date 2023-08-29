use comfy_table::Table;
use libsql::v2::Rows;

use crate::rows::{self, RowIterator};

pub fn print_rows(rows: &mut Rows) -> libsql::Result<()> {
    let mut table = Table::new();
    table.set_header(rows::column_names(rows));

    let column_count = rows.column_count() as usize;

    for row in RowIterator::new(rows) {
        table.add_row(rows::row_str_values(&row?, column_count)?);
    }

    println!("{table}");

    Ok(())
}
