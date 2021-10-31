local M = {}

---@param bang boolean @bang searches without any filtering
---@param initial_text string @pre-fill text into the telescope picker
function M.search(bang, initial_text)
  require('dash.providers.telescope').build_picker(bang == true, initial_text):find()
end

--- see src/config.rs for all config keys
--- https://github.com/mrjones2014/dash.nvim/blob/master/src/config.rs
---@param config table
function M.setup(config)
  require('libdash_nvim').setup(config)
end

return M
