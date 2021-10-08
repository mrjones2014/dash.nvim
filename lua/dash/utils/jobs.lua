local M = {}

function M.build_cli_path(dash_app_path)
  -- gsub to remove trailing slash, if there is one, because we're adding one
  return (dash_app_path:gsub('(.)%/$', '%1')) .. '/Contents/Resources/dashAlfredWorkflow'
end

--- Search Dash.app for query, return stdout, stderr
---@param query string
---@return string, string
function M.run_search(query)
  local Job = require('plenary.job')
  local stdout = nil
  local stderr = nil
  local dash_app_path = require('dash.utils.config').config.dash_app_path
  local cli_path = M.build_cli_path(dash_app_path)
  Job
    :new({
      command = cli_path,
      args = { query },
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
