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

local function getResultsForFiletype(currentFileType, prompt, bang)
  local config = require('dash.utils.config').config
  local fileTypeKeywords = config.fileTypeKeywords[currentFileType]
  if bang == true or not fileTypeKeywords then
    -- filtering by filetype is disabled
    return getResults(prompt)
  end

  if fileTypeKeywords == true then
    prompt = currentFileType .. ':' .. prompt
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

local function finderFn(currentFileType, bang)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end

    return getResultsForFiletype(currentFileType, prompt, bang)
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

local function buildPickerTitle()
  local config = require('dash.utils.config').config
  local filetype = vim.bo.filetype
  local fileTypeKeywords = config.fileTypeKeywords[filetype]
  if not fileTypeKeywords then
    return 'Dash'
  end

  if fileTypeKeywords == true then
    return 'Dash - ' .. filetype
  end

  if type(fileTypeKeywords) == 'string' then
    return 'Dash - ' .. fileTypeKeywords
  end

  if type(fileTypeKeywords) == 'table' then
    return 'Dash - ' .. (vim.inspect(fileTypeKeywords)):gsub('\n', '')
  end

  return 'Dash'
end

function M.buildPicker(bang)
  local Picker = require('telescope.pickers')
  local Finder = require('telescope.finders')
  local Sorter = require('telescope.sorters')
  local finder = Finder.new_dynamic({
    fn = finderFn(vim.bo.filetype, bang),
    entry_maker = function(entry)
      return entry
    end,
    on_complete = {},
  })

  local picker = Picker:new({
    prompt_title = buildPickerTitle(),
    finder = finder,
    sorter = Sorter.get_generic_fuzzy_sorter(),
    attach_mappings = attachMappings,
  })

  return picker
end

return M
