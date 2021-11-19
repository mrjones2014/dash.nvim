#[macro_use]
extern crate lazy_static;

mod constants;
mod dash_app_connector;
mod dash_item;
mod dash_query;
mod lua_bindings;
mod query_builder;
mod search_engine;
mod url_handler;

use crate::lua_bindings::dash_lua_bindings;
use mlua::prelude::{Lua, LuaResult, LuaTable};

/// This creates the Lua module that is exported as `libdash_nvim`.
///
/// # Module Table Fields
///
/// - `config` - the current configuration table, see `src/lua_bindings/dash_config_binding.rs`
/// - `default_config` - a copy of the config table with the default values, this should be treated as read-only
/// - `setup` - the function used to set the configuration table, provided values will be **merged with the default config table**
/// - `query` - the main API function, returns a list table of `DashItem`s, see `src/dash_item.rs`
/// - `open_url` - used for search engine fallback, simply opens the provided URL in the default handler
/// - `DASH_APP_BASE_PATH` - a constant with a value of "/Applications/Dash.app"
/// - `DASH_APP_CLI_PATH` - a constant with a value of "/Contents/Resources/dashAlfredWorkflow"
#[mlua::lua_module]
pub fn libdash_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("config", dash_lua_bindings::create_config_table(lua)?)?;
    exports.set(
        "default_config",
        dash_lua_bindings::create_config_table(lua)?,
    )?;
    exports.set("setup", dash_lua_bindings::create_setup_function(lua)?)?;
    exports.set("query", dash_lua_bindings::create_query_function(lua)?)?;
    exports.set(
        "open_url",
        dash_lua_bindings::create_open_url_function(lua)?,
    )?;
    exports.set(
        "open_item",
        dash_lua_bindings::create_open_item_function(lua)?,
    )?;
    exports.set("DASH_APP_BASE_PATH", constants::DASH_APP_BASE_PATH)?;
    exports.set("DASH_APP_CLI_PATH", constants::DASH_APP_CLI_PATH)?;
    Ok(exports)
}
