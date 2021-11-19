use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable};

fn get_nvim_global(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua.globals().get("vim")?)
}

pub fn report_errors(lua: &Lua, errors: &Vec<String>) -> LuaResult<bool> {
    let nvim = self::get_nvim_global(lua)?;
    let nvim_api: LuaTable = nvim.get("api")?;
    let err_writeln: LuaFunction = nvim_api.get("nvim_err_writeln")?;
    for error in errors.into_iter() {
        err_writeln.call(String::from(error))?;
    }
    Ok(true)
}
