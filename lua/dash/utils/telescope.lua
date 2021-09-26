local M = {}

local function getResults(prompt)
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

local function getResultsForFiletype(currentFileType, prompt)
  local config = require('dash.utils.config').config
  local fileTypeKeywords = config.fileTypeKeywords[currentFileType]
  if not config.filterWithCurrentFileType or fileTypeKeywords == false then
    -- filtering by filetype is disabled
    return getResults(prompt)
  end

  if type(fileTypeKeywords) == 'string' then
    prompt = fileTypeKeywords .. ':' .. prompt
    return getResults(prompt)
  end

  if type(fileTypeKeywords) == 'table' then
    local tableUtils = require('dash.utils.tables')
    local results = {}
    for _, value in ipairs(fileTypeKeywords) do
      local keywordPrompt = value .. ':' .. prompt
      results = tableUtils.concatArrays(results, getResults(keywordPrompt))
    end
    return results
  end
end

local function finderFn(currentFileType)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end

    return getResultsForFiletype(currentFileType, prompt)
  end
end

local function attachMappings(_, map)
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

function M.buildPicker()
  local Picker = require('telescope.pickers')
  local Finder = require('telescope.finders')
  local Sorter = require('telescope.sorters')
  local finder = Finder.new_dynamic({
    fn = finderFn(vim.bo.filetype),
    entry_maker = function(entry)
      return entry
    end,
    on_complete = {},
  })

  local picker = Picker:new({
    prompt_title = 'Dash',
    finder = finder,
    sorter = Sorter.get_generic_fuzzy_sorter(),
    attach_mappings = attachMappings,
  })

  return picker
end

return M
