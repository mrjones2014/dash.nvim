local M = {}

function M.finderFn(prompt)
  if not prompt or #prompt == 0 then
    return {}
  end
  local result = require('dash.utils.jobs').runSearch(prompt)
  local stdout = result.stdout
  local stderr = result.stderr

  if stdout ~= nil then
    local xmlUtils = require('dash.utils.xml')
    return xmlUtils.transformItems(xmlUtils.parse(stdout))
  end

  if stderr ~= nil then
    print(stderr)
    return {}
  end

  print('Failed to execute Dash.app query')
  return {}
end

function M.entryMaker(entry)
  return {
    value = entry[2],
    display = entry[1],
    ordinal = entry[1],
  }
end

function M.attachMappings(_, map)
  map('i', '<CR>', function(buffnr)
    local entry = require('telescope.actions').get_selected_entry()
    if not entry then
      return
    end
    require('dash.utils.jobs').openQuery(entry.value)
    require('telescope.actions').close(buffnr)
  end)
  return true
end

return M
