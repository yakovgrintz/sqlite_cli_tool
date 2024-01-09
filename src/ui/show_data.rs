use rusqlite::Connection;
use crate::db_func::get_data::get_head_table;

pub(crate) fn show(conn:&Connection,table_name: &str)->rusqlite::Result<()>{
    let data = get_head_table(conn, table_name)?;
    for row in data {
        println!("{}", row);
    }
    Ok(())
}