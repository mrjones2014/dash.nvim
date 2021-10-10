local M = {}

local dash_nvim_cli_path = vim.g.dash_root_dir .. require('dash.constants').dash_nvim_bin_path

--- Run queries and return stdout, stderr, in JSON format
---@param queries table @a table containing all queries to run as strings
---@return string, string
function M.run_queries(queries)
  local Job = require('plenary.job')
  local stdout = nil
  local stderr = nil
  local dash_app_path = require('dash.utils.config').config.dash_app_path
  table.insert(queries, 1, dash_app_path)
  table.insert(queries, 1, '-c')

  Job
    :new({
      command = dash_nvim_cli_path,
      args = queries,
      cwd = vim.fn.getcwd(),
      enabled_recording = true,
      on_exit = function(j, return_val)
        if return_val .. '' == '0' then
          stdout = j:result()
        else
          stderr = j:result()
        end
      end,
    })
    :sync()

  local strings = require('dash.utils.strings')
  return strings.trim_trailing_newlines(strings.join_list_to_string(stdout)),
    strings.trim_trailing_newlines(strings.join_list_to_string(stderr))
end

--- Open the query in Dash.app
---@param query string
function M.open_query(query)
  local Job = require('plenary.job')

  Job
    :new({
      command = 'open',
      args = { '-g', ('dash-workflow-callback://' .. require('dash.utils.strings').urlencode(query)) },
    })
    :start()
end

return M
