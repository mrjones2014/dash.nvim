pub mod dash_config_binding;
pub mod dash_query_binding;
pub mod nvim;
pub mod url_handler_binding;

/// This module handles creating Lua bindings to Rust internals.
pub mod dash_lua_bindings {
    use crate::lua_bindings::dash_config_binding;
    use crate::lua_bindings::dash_query_binding;
    use crate::lua_bindings::url_handler_binding;
    use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable};

    /// Creates a Lua table representing the default configuration.
    pub fn create_config_table(lua: &Lua) -> LuaResult<LuaTable> {
        Ok(dash_config_binding::get_default(lua))
    }

    /// Creates a Lua function `setup(config)` to set the module configuration.
    pub fn create_setup_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(dash_config_binding::setup)?)
    }

    /// Creates a Lua function `query(params)` to run the specified queries.
    pub fn create_query_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(dash_query_binding::query)?)
    }

    /// Creates a Lua function `open_url(url)` to open URLs. This is used
    /// for the search engine fallback.
    pub fn create_open_url_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(url_handler_binding::open_url)?)
    }

    /// Creates a Lua function `open_item(selected_item)` to open the selected `DashItem`
    /// in Dash.app.
    pub fn create_open_item_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(dash_query_binding::open_item)?)
    }
}
