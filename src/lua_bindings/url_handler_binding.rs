use crate::url_handler;
use mlua::prelude::{Lua, LuaResult};

/// Creates a Lua function to simply open a URL.
/// The Lua function takes a URL string as its argument.
pub fn open_url(_: &Lua, url: String) -> LuaResult<()> {
    url_handler::open_url(url);
    Ok(())
}
