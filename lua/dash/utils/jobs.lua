local M = {}

--- Open the query in Dash.app
---@param item_num string
function M.open_item(item_num)
  local Job = require('plenary.job')

  Job
    :new({
      command = 'open',
      args = { '-g', ('dash-workflow-callback://' .. item_num) },
    })
    :start()
end

return M
