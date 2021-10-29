use mlua::prelude::*;

pub fn build_query(
    lua: &Lua,
    search_text: &str,
    buffer_type: &str,
    bang: bool,
    file_type_keywords_table: LuaTable,
) -> Vec<String> {
    let mut queries = Vec::new();
    if bang {
        queries.push(search_text.to_string());
        return queries;
    }

    let file_type_keywords: LuaValue =
        if file_type_keywords_table.contains_key(buffer_type).unwrap() {
            file_type_keywords_table.get(buffer_type).unwrap()
        } else {
            mlua::Value::Boolean(false)
        };

    if file_type_keywords.type_name() == "boolean" {
        if file_type_keywords.eq(&mlua::Value::Boolean(true)) {
            queries.push(format!("{}:{}", buffer_type, search_text));
            return queries;
        }

        // otherwise it's false, and filtering by the buffer type is disabled
        queries.push(search_text.to_string());
        return queries;
    }

    if file_type_keywords.type_name() == "table" {
        let keywords_table: LuaTable = LuaTable::from_lua(file_type_keywords, lua).unwrap();
        for pair in keywords_table.pairs::<Option<String>, String>().into_iter() {
            queries.push(format!("{}:{}", pair.unwrap().1, search_text));
        }

        return queries;
    }

    // if all else fails, just return the search_text as the only query
    queries.push(search_text.to_string());
    return queries;
}
