use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable};

fn get_nvim_global(lua: &Lua) -> LuaResult<LuaTable> {
    lua.globals().get("vim")
}

/// Rust binding to call Neovim's Lua function `vim.api.nvim_err_writeln` from Rust.
/// Takes a `Vec<String>` and reports each one as an error in Neovim's error
/// reporting mechanism.
pub fn report_errors(lua: &Lua, errors: &[String]) -> LuaResult<bool> {
    let nvim = self::get_nvim_global(lua)?;
    let nvim_api: LuaTable = nvim.get("api")?;
    let err_writeln: LuaFunction = nvim_api.get("nvim_err_writeln")?;
    for error in errors.iter() {
        err_writeln.call(String::from(error))?;
    }
    Ok(true)
}
