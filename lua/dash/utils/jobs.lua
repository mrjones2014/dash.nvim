local M = {}

function M.runSearch(query)
  local Job = require('plenary.job')
  local stdout = nil
  local stderr = nil
  -- gsub to remove trailing slash, if there is one, because we're adding one
  local dashAppPath = require('dash.utils.config').config.dashAppPath
  local cliPath = (dashAppPath:gsub('(.)%/$', '%1')) .. '/Contents/Resources/dashAlfredWorkflow'
  Job
    :new({
      command = cliPath,
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

  local stringUtils = require('dash.utils.strings')

  return {
    stdout = stringUtils.trimTrailingNewlines(stringUtils.joinListToString(stdout)),
    stderr = stringUtils.trimTrailingNewlines(stringUtils.joinListToString(stderr)),
  }
end

function M.openQuery(query)
  local Job = require('plenary.job')

  Job
    :new({
      command = 'open',
      args = { '-g', ('dash-workflow-callback://' .. require('dash.utils.strings').urlencode(query)) },
    })
    :start()
end

return M
