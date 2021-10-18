mod cli_runner;
use cli_runner::TelescopeItem;
use crossbeam::channel;
use futures::future::join_all;
use mlua::prelude::*;
use std::ffi::c_void;
use std::marker::{Send, Sync};
use tokio::runtime::Runtime;

async fn query<'a>(cli_path: &'a String, queries: &'a Vec<String>) -> Vec<TelescopeItem> {
    let mut results: Vec<TelescopeItem> = Vec::new();
    let mut futures = Vec::new();
    queries.iter().for_each(|query| {
        futures.push(cli_runner::run_query(&cli_path, &query));
    });

    let all_futures = join_all(futures);
    let futures_results = all_futures.await;
    futures_results.iter().for_each(|result| {
        if result.len() > 0 {
            results.append(&mut result.to_owned());
        }
    });

    return results;
}

struct LuaTableShim(*const c_void);
unsafe impl Send for LuaTableShim {}
unsafe impl Sync for LuaTableShim {}

pub struct QueryParams {
    pub cli_path: &'static String,
    pub queries: &'static Vec<String>,
}

impl mlua::UserData for QueryParams {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(_methods: &mut M) {}
}

impl Clone for QueryParams {
    fn clone(&self) -> Self {
        return QueryParams {
            cli_path: &self.cli_path.to_string(),
            queries: &self.queries.clone(),
        };
    }
}

fn query_sync(params: &'static QueryParams) -> Vec<TelescopeItem> {
    let (tx, rx) = channel::bounded(1);
    let handle = Runtime::new().unwrap().handle();
    handle.spawn(async move {
        let result_table = &query(&params.cli_path, &params.queries).await;
        let _ = tx.send(&result_table);
    });
    return rx.recv().unwrap().to_owned().to_vec();
}

fn query_sync_lua_table<'a>(lua: &'a Lua, params: QueryParams) -> LuaTable<'a> {
    let result_telescope_items = query_sync(&params);
    let mut lua_table_items: Vec<LuaTable> = Vec::new();
    let mut lua_result_list: LuaTable = lua.create_table().unwrap();
    let mut i = 1;
    result_telescope_items.iter().for_each(|result| {
        let result_lua_table = lua.create_table().unwrap();
        result_lua_table.set("value", result.value.to_string());
        result_lua_table.set("ordinal", result.ordinal.to_string());
        result_lua_table.set("display", result.display.to_string());
        result_lua_table.set("keyword", result.keyword.to_string());
        result_lua_table.set("query", result.query.to_string());

        lua_table_items.push(result_lua_table);
        lua_result_list.raw_insert(i, result_lua_table.to_owned());
    });

    return lua_result_list.to_owned();
}

#[mlua::lua_module]
fn dash_runner(lua: &Lua) -> LuaResult<LuaTable> {
    let mut exports = lua.create_table().unwrap();
    exports.set("query", lua.create_function(query_sync_lua_table).unwrap());
    return Ok(exports);
}
