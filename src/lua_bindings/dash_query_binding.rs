mod dash_query_binding {
    use crate::{constants::constants, lua_bindings::dash_nvim_config::dash_nvim_config};
    use mlua::prelude::{Lua, LuaResult, LuaTable};

    pub fn query(lua: &Lua) -> LuaResult<LuaTable> {
        let config = dash_nvim_config::get_runtime_instance(lua);
        let dash_app_path = config
            .get("dash_app_path")
            .unwrap_or(String::from(constants::DASH_APP_BASE_PATH));
    }
}
