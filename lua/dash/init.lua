local M = {}

function M.search()
  require('dash.utils.telescope').buildPicker():find()
end

function M.setup(config)
  require('dash.utils.config').setup(config)
end

return M
