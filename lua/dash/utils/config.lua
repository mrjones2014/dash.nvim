local M = {}

M.config = {
  dashAppPath = '/Applications/Dash.app',
  fileTypeKeywords = {
    dashboard = false,
    NvimTree = false,
    TelescopePrompt = false,
    terminal = false,
    packer = false,
  },
  filterWithCurrentFileType = true,
  searchJavascriptWithTypescript = true,
}

local function defaultBoolean(value, default)
  if value ~= nil then
    return value
  end
  return default
end

function M.setup(newConfig)
  newConfig = newConfig or {}
  M.config.dashAppPath = newConfig.dashAppPath or M.config.dashAppPath
  M.config.fileTypeKeywords = require('dash.utils.tables').mergeTables(
    M.config.fileTypeKeywords,
    newConfig.fileTypeKeywords or {}
  )
  M.config.filterWithCurrentFileType = defaultBoolean(newConfig.filterWithCurrentFileType, true)
  M.config.searchJavascriptWithTypescript = defaultBoolean(newConfig.searchJavascriptWithTypescript, true)
end

return M
