use rusqlite::{Connection, OpenFlags};

use crate::cli_runner::TelescopeItem;

fn query_docset(
    database: &str,
    query: &str,
    keyword: &str,
) -> Result<Vec<TelescopeItem>, rusqlite::Error> {
    let conn = Connection::open_with_flags(database, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    let ztoken_query = conn
        .prepare("SELECT ID, NAME, TYPE, PATH FROM SEARCHINDEX WHERE NAME LIKE '%?%'")
        .unwrap();
    let query_result = ztoken_query
        .query_map(&[query], |row| {
            let title = format!("{}: {}", row.get(2).unwrap(), row.get(1).unwrap());
            return Ok(TelescopeItem {
                value: row.get(0)?,
                ordinal: title,
                display: title,
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
