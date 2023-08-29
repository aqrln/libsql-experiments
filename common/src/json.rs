use libsql::v2::{Row, Rows};
use serde_json::json;
use thiserror::Error;

use crate::rows::RowIterator;

#[derive(Error, Debug)]
pub enum RowsToJsonError {
    #[error("serialization error")]
    JsonError(#[from] serde_json::Error),
    #[error("database error")]
    LibsqlError(#[from] libsql::Error),
}

pub fn to_string(rows: &mut Rows) -> Result<String, RowsToJsonError> {
    Ok(serde_json::to_string_pretty(&serialize(rows)?)?)
}

pub fn serialize(rows: &mut Rows) -> libsql::Result<serde_json::Value> {
    let column_count = rows.column_count() as usize;
    let mut result = Vec::with_capacity(column_count);

    for row in RowIterator::new(rows) {
        result.push(row_to_json(&row?, column_count)?);
    }

    Ok(result.into())
}

fn row_to_json(row: &Row, column_count: usize) -> libsql::Result<serde_json::Value> {
    let mut result = json!({});

    for i in 0..column_count {
        let column_name = row.column_name(i as i32).unwrap();
        let value = row.get_value(i as i32)?;
        result[column_name] = into_json(value);
    }

    Ok(result)
}

fn into_json(value: libsql::Value) -> serde_json::Value {
    match value {
        libsql::Value::Null => json!(null),
        libsql::Value::Integer(num) => num.into(),
        libsql::Value::Real(num) => num.into(),
        libsql::Value::Text(text) => text.into(),
        libsql::Value::Blob(blob) => blob.into(),
    }
}
