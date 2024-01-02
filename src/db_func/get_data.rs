use rusqlite::{Connection};
pub(crate) fn get_head_table(conn: &Connection, table_name: &str) -> rusqlite::Result<Vec<String>> {
    let sql = format!("SELECT * FROM {} LIMIT 5", table_name);
    let mut stmt = conn.prepare(&sql)?;

    let mut data = Vec::new();
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let column_data: String = row.get(0)?;
        data.push(column_data);
    }

    Ok(data)
}



pub(crate) fn get_data_from_table(conn:&Connection, table_name:&str, limit : u64)->rusqlite::Result<Vec<String>>{
todo!()}
