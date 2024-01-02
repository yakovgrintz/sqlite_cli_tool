use rusqlite::Connection;
pub(crate) fn get_head_table(conn:&Connection, table_name:&str)->rusqlite::Result<Vec<String>>{

    todo!(after initialze get_data_from_table change code to call it and not execute itself);
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
pub(crate) fn get_data_from_table(conn:&Connection, table_name:&str, limit : u64)->rusqlite::Result<Vec<String>>{
todo!()}
