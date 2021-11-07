mod cli_runner;
mod config;
mod constants;
mod previews;
mod query_builder;
mod query_runner;
mod search_engine_fallback;

use config::{init_config, setup};
use mlua::prelude::{Lua, LuaResult, LuaTable};
use previews::{get_browsh_path, get_preview_cmd};
use query_builder::build_query;
use query_runner::{open_item, open_search_engine, query};

#[mlua::lua_module]
pub fn libdash_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table().unwrap();
    exports
        .set("query", lua.create_function(query).unwrap())
        .unwrap();
    exports
        .set("open_item", lua.create_function(open_item).unwrap())
        .unwrap();
    exports
        .set(
            "open_search_engine",
            lua.create_function(open_search_engine).unwrap(),
        )
        .unwrap();
    exports
        .set("build_query", lua.create_function(build_query).unwrap())
        .unwrap();
    exports
        .set(
            "get_browsh_path",
            lua.create_function(get_browsh_path).unwrap(),
        )
        .unwrap();
    exports
        .set(
            "get_preview_cmd",
            lua.create_function(get_preview_cmd).unwrap(),
        )
        .unwrap();
    exports.set("config", init_config(lua)).unwrap();
    exports.set("default_config", init_config(lua)).unwrap();
    exports
        .set("setup", lua.create_function(setup).unwrap())
        .unwrap();
    exports
        .set("DASH_APP_BASE_PATH", constants::DASH_APP_BASE_PATH)
        .unwrap();
    exports
        .set("DASH_APP_CLI_PATH", constants::DASH_APP_CLI_PATH)
        .unwrap();
    return Ok(exports);
}
