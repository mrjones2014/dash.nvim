local M = {}

function M.search(bang)
  require('dash.utils.telescope').build_picker(bang == true):find()
end

function M.setup(config)
  require('dash.utils.config').setup(config)
end

return M
