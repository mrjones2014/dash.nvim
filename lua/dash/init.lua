local M = {}

---@param bang boolean @bang searches without any filtering
function M.search(bang)
  require('dash.utils.telescope').build_picker(bang == true):find()
end

---@param config Config
function M.setup(config)
  require('dash.utils.config').setup(config)
end

return M
