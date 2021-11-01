use std::os::raw::c_double;
use std::process::Command;
use std::str::FromStr;

use crate::cli_runner::run_query;
use crate::cli_runner::TelescopeItem;
use crate::config::get_config_table;
use crate::constants::DASH_APP_BASE_PATH;
use crate::constants::DASH_APP_CLI_PATH;
use crate::query_builder::build_query;
use crate::search_engine_fallback::get_search_engine_url;
use crate::search_engine_fallback::SearchEngine;
use crossbeam::channel;
use futures::future::join_all;
use mlua::prelude::*;
use tokio::runtime::Runtime;

async fn query_async<'a>(cli_path: &'a str, queries: &'a Vec<String>) -> Vec<TelescopeItem> {
    let mut results: Vec<TelescopeItem> = Vec::new();
    let mut futures = Vec::new();
    for i in 0..queries.len() {
        let query = queries.get(i).unwrap();
        futures.push(run_query(&cli_path, &query));
    }

    let all_futures = join_all(futures);
    let futures_results = all_futures.await;
    futures_results.iter().for_each(|result| {
        if result.len() > 0 {
            results.append(&mut result.to_owned());
        }
    });

    return results;
}

fn query_sync(cli_path: String, queries: Vec<String>) -> Vec<TelescopeItem> {
    let (tx, rx) = channel::bounded(1);
    let runtime = Runtime::new().unwrap();
    let handle = runtime.handle();
    handle.spawn(async move {
        let result_table = &query_async(&cli_path, &queries).await;
        let _ = tx.send(result_table.clone());
    });
    return rx.recv().unwrap().to_owned().to_vec();
}

fn search_engine_human_name(search_engine: &SearchEngine) -> String {
    return match search_engine {
        SearchEngine::DDG => "DuckDuckGo".to_string(),
        SearchEngine::STARTPAGE => "StartPage".to_string(),
        SearchEngine::GOOGLE => "Google".to_string(),
    };
}

fn run_query_sync<'a>(
    lua: &'a Lua,
    (cli_path, queries, search_engine_fallback, search_text): (
        String,
        Vec<String>,
        SearchEngine,
        String,
    ),
) -> Result<LuaTable<'a>, LuaError> {
    let result_telescope_items = query_sync(cli_path.to_string(), queries);
    let lua_result_list: LuaTable = lua.create_table().unwrap();

    let mut i = 1;
    for result in result_telescope_items.iter() {
        let result_lua_table = &lua.create_table().unwrap();
        result_lua_table
            .set("value", result.value.to_string())
            .unwrap();
        result_lua_table
            .set("ordinal", result.ordinal.to_string())
            .unwrap();
        result_lua_table
            .set("display", result.display.to_string())
            .unwrap();
        result_lua_table
            .set("keyword", result.keyword.to_string())
            .unwrap();
        result_lua_table
            .set("query", result.query.to_string())
            .unwrap();

        lua_result_list
            .raw_insert(i, result_lua_table.to_owned())
            .unwrap();
        i = i + 1;
    }

    if result_telescope_items.len() == 0 {
        let search_engine_item_table = lua.create_table().unwrap();
        search_engine_item_table
            .set(
                "value",
                get_search_engine_url(&search_engine_fallback, &search_text),
            )
            .unwrap();
        search_engine_item_table.set("ordinal", "1").unwrap();
        search_engine_item_table
            .set(
                "display",
                format!(
                    "Search with {}: {}",
                    search_engine_human_name(&search_engine_fallback),
                    search_text
                ),
            )
            .unwrap();
        search_engine_item_table.set("is_fallback", true).unwrap();
        lua_result_list
            .raw_insert(1, search_engine_item_table)
            .unwrap();
    }

    return Ok(lua_result_list.to_owned());
}

pub fn query<'a>(
    lua: &'a Lua,
    (search_text, current_buffer_type, bang): (String, String, bool),
) -> Result<LuaTable<'a>, LuaError> {
    // compute query params from config
    let config = get_config_table(lua);
    let dash_app_base_path: String = config
        .get("dash_app_path")
        .unwrap_or(DASH_APP_BASE_PATH.to_string());
    let cli_path: String = format!("{}{}", dash_app_base_path, DASH_APP_CLI_PATH);
    let search_engine_string: String = config.get("search_engine").unwrap_or("ddg".to_string());
    let search_engine = SearchEngine::from_str(&search_engine_string).unwrap();
    let file_type_keywords: LuaTable = config.get("file_type_keywords").unwrap();

    return run_query_sync(
        lua,
        (
            cli_path,
            build_query(
                lua,
                (
                    search_text.to_string(),
                    current_buffer_type,
                    bang,
                    file_type_keywords,
                ),
            )
            .unwrap(),
            search_engine,
            search_text,
        ),
    );
}

pub fn open_item(_: &Lua, item_num: c_double) -> Result<bool, LuaError> {
    Command::new("open")
        .args(&["-g", &format!("dash-workflow-callback://{}", item_num)])
        .output()
        .expect("Failed to open Dash.app");
    return Ok(true);
}

pub fn open_search_engine(_: &Lua, url: String) -> Result<bool, LuaError> {
    Command::new("open")
        .args(&[&url])
        .output()
        .expect("Failed to open URL");
    return Ok(true);
}
