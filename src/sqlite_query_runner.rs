use std::path::Path;

use rusqlite::{Connection, OpenFlags};

use crate::cli_runner::DashItem;

fn query_docset(
    database: &str,
    query: &str,
    keyword: &str,
) -> Result<Vec<DashItem>, rusqlite::Error> {
    if !Path::new(database).exists() {
        return Ok(Vec::new());
    }

    let conn = Connection::open_with_flags(database, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    let mut sql_query = conn
        .prepare("SELECT ID, NAME, TYPE, PATH FROM SEARCHINDEX WHERE NAME LIKE '%?%'")
        .unwrap();
    let query_result = sql_query
        .query_map(&[query], |row| {
            let name: String = row.get(1)?;
            let item_type: String = row.get(2)?;
            let title = format!("{}: {}", item_type, name);
            return Ok(DashItem {
                value: row.get(0)?,
                ordinal: title.to_string(),
                display: title.to_string(),
                keyword: keyword.to_string(),
                query: query.to_string(),
            });
        })
        .unwrap();
    let mut result_vec = Vec::new();
    for item in query_result.into_iter() {
        result_vec.push(item.unwrap());
    }
    return Ok(result_vec);
}
