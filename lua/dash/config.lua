---@class DashConfigManager
local M = {}

---@class DashConfig
M.config = {
  dash_app_path = require('libdash_nvim').DASH_APP_BASE_PATH,
  debounce = 0,
  file_type_keywords = {
    dashboard = false,
    NvimTree = false,
    TelescopePrompt = false,
    terminal = false,
    packer = false,
    javascript = { 'javascript', 'nodejs' },
    typescript = { 'typescript', 'javascript', 'nodejs' },
    typescriptreact = { 'typescript', 'javascript', 'react' },
    javascriptreact = { 'javascript', 'react' },
    swift = true,
    csharp = true,
    actionscript = true,
    applescript = true,
    bash = true,
    sh = 'bash',
    c = true,
    cpp = true,
    php = true,
    clojure = true,
    make = 'cmake',
    coffeescript = true,
    lisp = true,
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
    julia = true,
    latex = true,
    less = true,
    lua = true,
    sql = 'mysql',
    ocaml = true,
    perl = true,
    pug = true,
    python = true,
    r = true,
    ruby = true,
    rust = true,
    sass = true,
    scss = 'sass',
    scala = true,
    stylus = true,
    svg = true,
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
  M.config.dash_app_path = new_config.dash_app_path or M.config.dash_app_path
  M.config.debounce = new_config.debounce or M.config.debounce

  if new_config.file_type_keywords == false then
    M.config.file_type_keywords = {}
  elseif new_config.file_type_keywords then
    for key, value in pairs(new_config.file_type_keywords) do
      M.config.file_type_keywords[key] = value
    end
  end
end

return M
