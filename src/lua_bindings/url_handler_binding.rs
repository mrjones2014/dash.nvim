pub mod url_handler_binding {
    use crate::url_handler::url_handler;
    use mlua::prelude::{Lua, LuaResult};

    pub fn open_url(lua: &Lua, url: String) -> LuaResult<()> {
        url_handler::open_url(url);
        Ok(())
    }
}
