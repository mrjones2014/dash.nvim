---@class ConfigManager
local M = {}

---@class Config
M.config = {
  dash_app_path = '/Applications/Dash.app',
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

--- Merge user config with default config
---@param new_config Config
function M.setup(new_config)
  new_config = new_config or {}
  M.config.dash_app_path = new_config.dash_app_path or M.config.dash_app_path

  if new_config.file_type_keywords == false then
    M.config.file_type_keywords = {}
  else
    M.config.file_type_keywords = require('dash.utils.tables').merge_tables(
      M.config.file_type_keywords,
      new_config.file_type_keywords or {}
    )
  end
end

return M
