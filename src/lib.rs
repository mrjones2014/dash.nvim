mod cli_runner;
mod constants;
mod query_runner;
mod search_engine_fallback;

use mlua::prelude::{Lua, LuaResult, LuaTable};
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
        .set("DASH_APP_BASE_PATH", constants::DASH_APP_BASE_PATH)
        .unwrap();
    exports
        .set("DASH_APP_CLI_PATH", constants::DASH_APP_CLI_PATH)
        .unwrap();
    return Ok(exports);
}
