use crate::config::get_config_table;
use mlua::prelude::{LuaError, LuaTable};
use mlua::Lua;
use std::{env, fs};

fn is_browsh_in_path() -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/browsh", p);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    return false;
}

pub fn get_browsh_path(lua: &Lua, _: ()) -> Result<String, LuaError> {
    let config = get_config_table(lua);
    let mut browsh_path = config.get("browsh_path").unwrap_or("browsh".to_string());
    if browsh_path == "browsh" && !is_browsh_in_path() {
        return Ok("".to_string());
    }

    if browsh_path.starts_with("~/") {
        let home = env::var("HOME").unwrap_or("".to_string());
        if home == "" {
            return Ok("".to_string());
        }
        browsh_path = format!("{}/{}", &home, browsh_path.split_at(1).1);
    }

    return Ok(browsh_path.to_string());
}

pub fn get_preview_cmd(lua: &Lua, preview_url: String) -> Result<LuaTable, LuaError> {
    let browsh_path = get_browsh_path(lua, ()).unwrap();
    let cmd_with_args = lua.create_table().unwrap();
    cmd_with_args.raw_insert(1, browsh_path).unwrap();
    cmd_with_args.raw_insert(2, "--startup-url").unwrap();
    cmd_with_args.raw_insert(3, preview_url).unwrap();
    return Ok(cmd_with_args);
}
