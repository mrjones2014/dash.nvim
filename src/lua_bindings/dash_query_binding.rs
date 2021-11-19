use crate::lua_bindings::nvim;
use crate::{
    constants,
    dash_query::{self, QueryError},
    lua_bindings::dash_config_binding,
    query_builder,
    search_engine::SearchEngine,
    url_handler,
};
use mlua::{
    prelude::{Lua, LuaError, LuaResult, LuaString, LuaTable, LuaValue},
    FromLua,
};
use std::os::raw::c_double;
use std::{convert::TryInto, str::FromStr};

impl From<QueryError> for LuaError {
    fn from(e: QueryError) -> Self {
        LuaError::RuntimeError(format!("{}", e))
    }
}

fn get_effective_file_type_keywords(
    lua: &Lua,
    buffer_type: &str,
    file_type_keywords_config: LuaValue,
) -> LuaResult<Vec<String>> {
    let mut file_type_keywords: Vec<String> = Vec::new();

    match file_type_keywords_config.type_name() {
        "boolean" => {
            file_type_keywords = if file_type_keywords_config.eq(&mlua::Value::Boolean(true)) {
                vec![String::from(buffer_type)]
            } else {
                vec![]
            }
        }
        "string" => {
            file_type_keywords = vec![LuaString::from_lua(file_type_keywords_config, lua)?
                .to_str()?
                .to_string()]
        }
        "table" => {
            let keywords_table = LuaTable::from_lua(file_type_keywords_config, lua)?;
            for pair in keywords_table.pairs::<Option<String>, String>().into_iter() {
                let value = pair?.1;
                file_type_keywords.push(value);
            }
        }
        _ => {
            return Err(LuaError::RuntimeError(format!(
                "Invalid type for file_type_keywords config value: {}",
                file_type_keywords_config.type_name()
            )))
        }
    }

    Ok(file_type_keywords)
}

fn get_search_params(
    lua: &Lua,
    params: LuaTable,
) -> LuaResult<(String, Vec<String>, SearchEngine)> {
    let search_text = params.get("search_text").unwrap_or(String::from(""));
    if search_text.len() == 0 {
        return Ok((String::from(""), Vec::new(), SearchEngine::DDG));
    }

    let config = dash_config_binding::get_runtime_instance(lua);
    let dash_app_path = config
        .get("dash_app_path")
        .unwrap_or(String::from(constants::DASH_APP_BASE_PATH));
    let cli_path = format!("{}{}", dash_app_path, constants::DASH_APP_CLI_PATH);
    let search_engine_string = config.get("search_engine").unwrap_or(String::from("ddg"));
    let search_engine = SearchEngine::from_str(&search_engine_string).unwrap();
    let file_type_keywords_tbl = config
        .get("file_type_keywords")
        .unwrap_or(lua.create_table().unwrap());
    let buffer_type = params.get("buffer_type").unwrap_or(String::from("No Name"));
    let ignore_keywords = params.get("ignore_keywords").unwrap_or(false);
    let file_type_keywords_tbl_value: LuaValue = file_type_keywords_tbl
        .get(String::from(&buffer_type))
        .unwrap_or(LuaValue::Table(lua.create_table().unwrap()));
    let file_type_keywords =
        get_effective_file_type_keywords(lua, &buffer_type, file_type_keywords_tbl_value)
            .unwrap_or(Vec::new());
    let queries = if ignore_keywords {
        vec![search_text]
    } else {
        query_builder::build_queries(search_text, &file_type_keywords)
    };

    Ok((cli_path, queries, search_engine))
}

/// Lua binding to `dash_query::run_queries_parallel`.
///
/// Creates a Lua function which takes a table as its only parameter. The
/// table needs the following keys:
///
/// - `search_text` - the search text entered by the user
/// - `buffer_type` - the current buffer type, this will be used to determine filter keywords from config
/// - `ignore_keywords` - disables filtering by keywords if true
pub fn query<'a>(lua: &'a Lua, params: LuaTable) -> LuaResult<LuaTable<'a>> {
    let (cli_path, queries, search_engine) = get_search_params(lua, params.to_owned())?;
    let (results, errors) = dash_query::run_queries_parallel(cli_path, queries, search_engine);

    if errors.len() > 0 {
        // don't fail the whole query method on the off chance
        // that nvim::report_errors returns an error, just ignore it
        let _ = nvim::report_errors(lua, &errors);
    }

    let results_tbl = lua.create_table()?;
    for i in 0..results.len() {
        let item = &results[i];
        let tbl = lua.create_table().unwrap();
        tbl.set("value", &*item.value).unwrap();
        tbl.set("ordinal", &*item.ordinal).unwrap();
        tbl.set("display", &*item.display).unwrap();
        tbl.set("keyword", &*item.keyword).unwrap();
        tbl.set("query", &*item.query).unwrap();
        tbl.set("is_fallback", item.is_fallback).unwrap();
        // Lua tables are indexed from 1, not 0
        results_tbl
            .raw_insert((i + 1).try_into().unwrap(), tbl)
            .unwrap();
    }

    Ok(results_tbl)
}

/// Creates a Lua function to open a `DashItem` in Dash.app
/// once selected. The Lua function accepts a single `DashItem`
/// as its parameter and opens it in Dash.app.
pub fn open_item(lua: &Lua, item: LuaTable) -> LuaResult<()> {
    let id: c_double = item.get("value").unwrap_or(-1.0);
    if id < 0.0 {
        // No item was actually selected
        return Ok(());
    }

    let config = dash_config_binding::get_runtime_instance(lua);
    let dash_app_path = config
        .get("dash_app_path")
        .unwrap_or(String::from(constants::DASH_APP_BASE_PATH));
    let cli_path = format!("{}{}", dash_app_path, constants::DASH_APP_CLI_PATH);
    let query = item.get("query").unwrap_or(String::from(""));
    dash_query::run_query_sync(&cli_path, &query)?;
    url_handler::open_url(format!("{}{}", constants::DASH_CALLBACK_PROTO, id));
    Ok(())
}
