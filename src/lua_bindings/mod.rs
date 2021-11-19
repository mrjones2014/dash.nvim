pub mod dash_config_binding;
pub mod dash_query_binding;
pub mod nvim;
pub mod url_handler_binding;

pub mod dash_lua_bindings {
    use crate::lua_bindings::dash_config_binding;
    use crate::lua_bindings::dash_query_binding;
    use crate::lua_bindings::url_handler_binding;
    use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable};

    pub fn create_config_table(lua: &Lua) -> LuaResult<LuaTable> {
        Ok(dash_config_binding::get_default(lua))
    }

    pub fn create_setup_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(dash_config_binding::setup)?)
    }

    pub fn create_query_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(dash_query_binding::query)?)
    }

    pub fn create_open_url_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(url_handler_binding::open_url)?)
    }

    pub fn create_open_item_function(lua: &Lua) -> LuaResult<LuaFunction> {
        Ok(lua.create_function(dash_query_binding::open_item)?)
    }
}
