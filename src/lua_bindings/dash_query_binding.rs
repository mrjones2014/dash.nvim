mod dash_query_binding {
    use crate::{
        constants::constants, dash_query::dash_query,
        lua_bindings::dash_nvim_config::dash_nvim_config, query_builder::query_builder,
        search_engine::SearchEngine,
    };
    use mlua::{
        prelude::{Lua, LuaError, LuaResult, LuaString, LuaTable, LuaValue},
        FromLua,
    };
    use std::{convert::TryInto, str::FromStr};

    fn get_effective_file_type_keywords(
        lua: &Lua,
        buffer_type: &str,
        file_type_keywords_config: LuaValue,
    ) -> LuaResult<Vec<String>> {
        let mut file_type_keywords: Vec<String> = Vec::new();

        match file_type_keywords_config.type_name() {
            "boolean" => {
                file_type_keywords = if file_type_keywords_config.eq(&mlua::Value::Boolean(true)) {
                    vec![String::from(buffer_type)]
                } else {
                    vec![]
                }
            }
            "string" => {
                file_type_keywords = vec![LuaString::from_lua(file_type_keywords_config, lua)?
                    .to_str()?
                    .to_string()]
            }
            "table" => {
                let keywords_table = LuaTable::from_lua(file_type_keywords_config, lua)?;
                for pair in keywords_table.pairs::<Option<String>, String>().into_iter() {
                    let value = pair?.1;
                    file_type_keywords.push(value);
                }
            }
            _ => {
                return Err(LuaError::RuntimeError(format!(
                    "Invalid type for file_type_keywords config value: {}",
                    file_type_keywords_config.type_name()
                )))
            }
        }

        Ok(file_type_keywords)
    }

    fn get_search_params(
        lua: &Lua,
        params: LuaTable,
    ) -> LuaResult<(String, Vec<String>, SearchEngine)> {
        let search_text = params.get("search_text").unwrap_or(String::from(""));
        if search_text.len() == 0 {
            return Ok((String::from(""), Vec::new(), SearchEngine::DDG));
        }

        let config = dash_nvim_config::get_runtime_instance(lua);
        let dash_app_path = config
            .get("dash_app_path")
            .unwrap_or(String::from(constants::DASH_APP_BASE_PATH));
        let cli_path = format!("{}{}", dash_app_path, constants::DASH_APP_CLI_PATH);
        let search_engine_string = config.get("search_engine").unwrap_or(String::from("ddg"));
        let search_engine = SearchEngine::from_str(&search_engine_string).unwrap();
        let file_type_keywords_tbl = config
            .get("file_type_keywords")
            .unwrap_or(lua.create_table().unwrap());
        let buffer_type = params.get("buffer_type").unwrap_or(String::from("No Name"));
        let file_type_keywords_tbl_value: LuaValue = file_type_keywords_tbl
            .get(String::from(&buffer_type))
            .unwrap_or(LuaValue::Table(lua.create_table().unwrap()));
        let file_type_keywords =
            get_effective_file_type_keywords(lua, &buffer_type, file_type_keywords_tbl_value)?;

        Ok((cli_path, file_type_keywords, search_engine))
    }

    pub fn query<'a>(lua: &'a Lua, params: LuaTable) -> LuaResult<LuaTable<'a>> {
        let (cli_path, queries, search_engine) = get_search_params(lua, params)?;
        let (results, _errors) =
            dash_query::run_queries_parallel(&cli_path, &queries, &search_engine);

        let results_tbl = lua.create_table()?;
        for i in 0..results.len() {
            let item = &results[i];
            let tbl = lua.create_table().unwrap();
            tbl.set("value", &*item.value).unwrap();
            tbl.set("ordinal", &*item.ordinal).unwrap();
            tbl.set("display", &*item.display).unwrap();
            tbl.set("keyword", &*item.keyword).unwrap();
            tbl.set("query", &*item.query).unwrap();
            results_tbl.raw_insert(i.try_into().unwrap(), tbl);
        }

        Ok(results_tbl)
    }
}
