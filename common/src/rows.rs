use libsql::v2::{Row, Rows};

pub struct RowIterator<'a>(&'a mut Rows);

impl<'a> RowIterator<'a> {
    pub fn new(rows: &'a mut Rows) -> Self {
        Self(rows)
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = libsql::Result<Row>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().transpose()
    }
}

pub fn column_names(rows: &Rows) -> Vec<&str> {
    let mut cols = Vec::with_capacity(rows.column_count() as usize);
    for i in 0..rows.column_count() {
        cols.push(rows.column_name(i).unwrap());
    }
    cols
}

pub fn row_str_values(row: &Row, column_count: usize) -> libsql::Result<Vec<&str>> {
    let mut values = Vec::with_capacity(column_count);
    for i in 0..column_count {
        values.push(row.get_str(i as i32)?);
    }
    Ok(values)
}
