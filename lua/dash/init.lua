local M = {}

local telescopeRegistered = false

function M.registerWithTelescope()
  if telescopeRegistered then
    return
  end
  require('telescope._extensions').register({
    setup = M.setup,
    exports = {
      M.search,
    },
  })
end

function M.search(bang)
  require('dash.utils.telescope').buildPicker(bang == true):find()
end

function M.setup(config)
  require('dash.utils.config').setup(config)
end

return M
