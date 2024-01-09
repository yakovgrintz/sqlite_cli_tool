use rusqlite::{Connection, Rows};
use rusqlite::types::ValueRef;

struct TableData {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}
fn fetch_column_names(rows: &Rows) -> Vec<String> {
    rows.column_names().into_iter().map(|name| name.to_string()).collect()
}
fn fetch_table_data(rows: &mut Rows) -> Result<TableData> {
    let headers = fetch_column_names(&rows);

    let mut data_rows: Vec<Vec<String>> = Vec::new();

    while let Some(row) = rows.next()? {
        let mut row_values = Vec::new();
        for i in 0..row.column_count() {
            let value = match row.get_ref_unwrap(i) {
                ValueRef::Null => "NULL".to_string(),
                ValueRef::Integer(i) => i.to_string(),
                ValueRef::Real(f) => f.to_string(),
                ValueRef::Text(t) => String::from_utf8_lossy(t).into_owned(),
                ValueRef::Blob(_) => String::from("[BLOB data]"),
            };
            row_values.push(value);
        }
        data_rows.push(row_values);
    }

    Ok(TableData { headers, rows: data_rows })
}
pub(crate) fn get_head_table(conn: &Connection, table_name: &str) -> rusqlite::Result<Rows> {
    let sql = format!("SELECT * FROM {} LIMIT 5", table_name);
    let mut stmt = conn.prepare(&sql)?;

    let mut rows = stmt.query([])?;


    Ok(rows)
}




