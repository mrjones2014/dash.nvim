local M = {}

local basePath = os.getenv('HOME') .. '/Library/Application Support/Alfred/Alfred.alfredpreferences/workflows'
local workflowPath = basePath .. '/user.workflow.5543FE45-6F33-4CDA-BC36-496472725DB2'
local cliPath = workflowPath .. '/dashAlfredWorkflow'

function M.runSearch(query)
  local Job = require('plenary.job')
  local stdout = nil
  local stderr = nil
  Job
    :new({
      command = cliPath,
      args = { query },
      cwd = workflowPath,
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

  return {
    stdout = M.trimTrailingNewlines(M.joinListToString(stdout)),
    stderr = M.trimTrailingNewlines(M.joinListToString(stderr)),
  }
end

function M.joinListToString(output)
  if not (type(output) == 'table') then
    return output
  end

  local str = ''
  for _, val in pairs(output) do
    str = str .. val .. '\n'
  end
  return str
end

function M.trimTrailingNewlines(str)
  if str == nil then
    return nil
  end
  local n = #str
  while n > 0 and str:find('^%s', n) do
    n = n - 1
  end
  return str:sub(1, n)
end

return M
