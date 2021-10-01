local M = {}

M.config = {
  dashAppPath = '/Applications/Dash.app',
  fileTypeKeywords = {
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

function M.setup(newConfig)
  newConfig = newConfig or {}
  M.config.dashAppPath = newConfig.dashAppPath or M.config.dashAppPath
  M.config.fileTypeKeywords = require('dash.utils.tables').mergeTables(
    M.config.fileTypeKeywords,
    newConfig.fileTypeKeywords or {}
  )
end

return M
