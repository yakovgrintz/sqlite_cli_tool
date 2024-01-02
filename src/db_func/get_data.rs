use rusqlite::Connection;
pub(crate) fn get_head_table(conn:&Connection, table_name:&str)->rusqlite::Result<Vec<String>>{
    let sql = format!("SELECT * FROM {} LIMIT 5", table_name);
    let mut stmt = conn.prepare(&sql)?;

    let rows = stmt.query_map(|row| {
        let column_data: String = row.get(0)?;
        Ok(column_data)
    })?;

    // Collect the data into a Vec<String>
    let mut data = Vec::new();
    for result in rows {
        let column_data = result?;
        data.push(column_data);
    }

    Ok(data)
}
