local M = {}

---@param bang boolean @bang searches without any filtering
---@param initial_text string @pre-fill text into the telescope picker
function M.search(bang, initial_text)
  require('dash.providers.telescope').build_picker(bang == true, initial_text):find()
end

---@param config DashConfig
function M.setup(config)
  require('dash.config').setup(config)
end

return M
