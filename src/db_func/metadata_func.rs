use rusqlite::Connection;

pub(crate) fn get_table_names(connection: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = connection.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let tables = stmt.query_map([], |row| row.get(0))?;
    tables.collect()
}


pub(crate) fn get_column_names(connection: &Connection, table_name: &str) -> rusqlite::Result<Vec<String>> {
    let mut stmt = connection.prepare(&format!("PRAGMA table_info({})", table_name))?;
    let columns = stmt.query_map([], |row| row.get(1))?;
    columns.collect()
}
