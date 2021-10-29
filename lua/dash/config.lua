---@class DashConfigManager
local M = {}

---@class DashConfig
M.config = {
  dash_app_path = require('libdash_nvim').DASH_APP_BASE_PATH,
  search_engine = 'ddg',
  debounce = 0,
  file_type_keywords = {
    -- plugin exclusions
    dashboard = false,
    NvimTree = false,
    TelescopePrompt = false,
    terminal = false,
    packer = false,

    -- filetypes, try to keep these alphabetical
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
    javascript = { 'javascript', 'nodejs' },
    javascriptreact = { 'javascript', 'react' },
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
    typescript = { 'typescript', 'javascript', 'nodejs' },
    typescriptreact = { 'typescript', 'javascript', 'react' },
    vim = true,
  },
}

local function deep_copy(tbl)
  local result = {}

  for k, v in pairs(tbl) do
    if type(v) == 'table' then
      result[k] = deep_copy(v)
    else
      result[k] = v
    end
  end

  return result
end

M.default_config = deep_copy(M.config)

--- Merge user config with default config
---@param new_config DashConfig
function M.setup(new_config)
  new_config = new_config or {}
  M.config.dash_app_path = new_config.dash_app_path or M.default_config.dash_app_path
  M.config.debounce = new_config.debounce or M.default_config.debounce
  M.config.search_engine = new_config.search_engine or M.default_config.search_engine

  if new_config.file_type_keywords == false then
    M.config.file_type_keywords = {}
  elseif new_config.file_type_keywords then
    for key, value in pairs(new_config.file_type_keywords) do
      M.config.file_type_keywords[key] = value
    end
  end
end

return M
