use std::os::raw::c_double;
use std::process::Command;

use crate::cli_runner::run_query;
use crate::cli_runner::TelescopeItem;
use crossbeam::channel;
use futures::future::join_all;
use mlua::prelude::*;
use tokio::runtime::Runtime;

async fn query<'a>(queries: &'a Vec<String>) -> Vec<TelescopeItem> {
    let mut results: Vec<TelescopeItem> = Vec::new();
    let mut futures = Vec::new();
    let cli_path = queries.get(0).unwrap();
    for i in 1..queries.len() {
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

fn query_sync(params: Vec<String>) -> Vec<TelescopeItem> {
    let (tx, rx) = channel::bounded(1);
    let runtime = Runtime::new().unwrap();
    let handle = runtime.handle();
    handle.spawn(async move {
        let result_table = &query(&params).await;
        let _ = tx.send(result_table.clone());
    });
    return rx.recv().unwrap().to_owned().to_vec();
}

pub fn run_query_sync<'a>(lua: &'a Lua, params: Vec<String>) -> Result<LuaTable<'a>, LuaError> {
    let result_telescope_items = query_sync(params);
    let mut lua_table_items: Vec<LuaTable> = Vec::new();
    let lua_result_list: LuaTable = lua.create_table().unwrap();
    let mut i = 1;
    result_telescope_items.iter().for_each(|result| {
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

        lua_table_items.push(result_lua_table.to_owned());
        lua_result_list
            .raw_insert(i, result_lua_table.to_owned())
            .unwrap();
        i = i + 1;
    });

    return Ok(lua_result_list.to_owned());
}

pub fn open_item(_: &Lua, item_num: c_double) -> Result<bool, LuaError> {
    Command::new("open")
        .args(&["-g", &format!("dash-workflow-callback://{}", item_num)])
        .output()
        .expect("Failed to open Dash.app");
    return Ok(true);
}
