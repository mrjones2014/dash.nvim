use crate::constants;
use mlua::prelude::{LuaError, LuaFunction, LuaTable, LuaValue};
use mlua::{FromLua, Lua};

pub fn get_config_table(lua: &Lua) -> LuaTable {
    let require: LuaFunction = lua.globals().get("require").unwrap();
    let module: LuaTable = require.call("libdash_nvim").unwrap();
    let config: LuaTable = module.get("config").unwrap();
    return config;
}

pub fn init_config(lua: &Lua) -> LuaTable {
    let config: LuaTable = lua
        .load(
            format!(
                r#"
{{
  dash_app_path = '{}',
  search_engine = 'ddg',
  debounce = 0,
  file_type_keywords = {{
    -- plugin excludes
    dashboard = false,
    NvimTree = false,
    TelescopePrompt = false,
    terminal = false,
    packer = false,
    fzf = false,

    -- filetypes, keep these ones alphabetical
    actionscript = true,
    applescript = true,
    bash = true,
    c = true,
    clojure = true,
    coffeescript = true,
    cpp = true,
    csharp = true,
    css = true,
    dart = true,
    dockerfile = 'docker',
    elixir = true,
    erlang = true,
    go = true,
    groovy = true,
    haml = true,
    handlebars = true,
    haskell = true,
    html = true,
    java = true,
    javascript = {{ 'javascript', 'nodejs' }},
    javascriptreact = {{ 'javascript', 'react' }},
    julia = true,
    latex = true,
    less = true,
    lisp = true,
    lua = true,
    make = 'cmake',
    ocaml = true,
    perl = true,
    php = true,
    pug = true,
    python = true,
    r = true,
    ruby = true,
    rust = true,
    sass = true,
    scala = true,
    scss = 'sass',
    sh = 'bash',
    sql = 'mysql',
    stylus = true,
    svg = true,
    swift = true,
    terraform = true,
    typescript = {{ 'typescript', 'javascript', 'nodejs' }},
    typescriptreact = {{ 'typescript', 'javascript', 'react' }},
    vim = true,
  }},
}}
"#,
                constants::DASH_APP_BASE_PATH
            )
            .as_str(),
        )
        .eval()
        .unwrap();
    return config;
}

pub fn setup<'a>(lua: &'a Lua, new_config: LuaTable) -> Result<LuaTable<'a>, LuaError> {
    let config_table: LuaTable = get_config_table(lua);

    let dash_app_path: String = new_config
        .get("dash_app_path")
        .unwrap_or(config_table.get("dash_app_path").unwrap());
    let debounce: String = new_config
        .get("debounce")
        .unwrap_or(config_table.get("debounce").unwrap());
    let search_engine: String = new_config
        .get("search_engine")
        .unwrap_or(config_table.get("search_engine").unwrap());

    config_table.set("dash_app_path", dash_app_path).unwrap();
    config_table.set("debounce", debounce).unwrap();
    config_table.set("search_engine", search_engine).unwrap();

    if new_config.contains_key("file_type_keywords").unwrap() {
        let keywords_config_value: LuaValue = new_config.get("file_type_keywords").unwrap();
        if keywords_config_value.type_name() == "boolean"
            && keywords_config_value.eq(&mlua::Value::Boolean(false))
        {
            config_table
                .set("file_type_keywords", lua.create_table().unwrap())
                .unwrap();
            return Ok(config_table);
        }

        if keywords_config_value.type_name() == "table" {
            let keywords_config_table: LuaTable = config_table.get("file_type_keywords").unwrap();
            let keywords_table: LuaTable = LuaTable::from_lua(keywords_config_value, lua).unwrap();
            for pair in keywords_table.pairs::<String, LuaValue>().into_iter() {
                let unwrapped = pair.unwrap();
                let keyword_key = unwrapped.0;
                let keyword_value: LuaValue = unwrapped.1;
                keywords_config_table
                    .set(keyword_key.to_string(), keyword_value)
                    .unwrap();
                config_table
                    .set("file_type_keywords", keywords_config_table)
                    .unwrap();
                return Ok(config_table);
            }
        }
    }

    return Ok(config_table);
}
