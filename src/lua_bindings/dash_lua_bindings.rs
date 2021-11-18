mod dash_lua_bindings {
    use crate::lua_bindings::dash_nvim_config::dash_nvim_config;
    use mlua::prelude::{Lua, LuaFunction, LuaTable};

    pub fn create_config_table(lua: &Lua) -> LuaTable {
        dash_nvim_config::get_default(lua)
    }

    pub fn create_setup_function(lua: &Lua) -> LuaFunction {
        lua.create_function(dash_nvim_config::setup).unwrap()
    }

    // pub fn create_query_function()
}
