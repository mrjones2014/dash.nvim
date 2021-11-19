use crate::url_handler;
use mlua::prelude::{Lua, LuaResult};

pub fn open_url(_: &Lua, url: String) -> LuaResult<()> {
    url_handler::open_url(url);
    Ok(())
}
