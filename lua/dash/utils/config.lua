local M = {}

M.config = {
  dashAppPath = '/Applications/Dash.app',
}

function M.setup(newConfig)
  newConfig = newConfig or {}
  M.config.dashAppPath = newConfig.dashAppPath or M.config.dashAppPath
end

return M
