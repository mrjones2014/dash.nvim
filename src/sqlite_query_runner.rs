use rusqlite::{Connection, OpenFlags};

use crate::cli_runner::TelescopeItem;

fn query_docset(database: &str, query: &str, keyword: &str) -> Result<Connection, _> {
    let conn = Connection::open_with_flags(database, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let ztoken_query =
        conn.prepare("SELECT ID, NAME, TYPE, PATH FROM SEARCHINDEX WHERE NAME LIKE '%?%'")?;
    let query_result = ztoken_query.query_map(&[query], |row| {
        let title = format!("{}: {}", row.get(2)?, row.get(1)?);
        return Ok(TelescopeItem {
            value: row.get(0)?,
            ordinal: title,
            display: title,
            keyword: keyword.to_string(),
            query: query.to_string(),
        });
    });
}
