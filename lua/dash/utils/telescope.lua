local M = {}

local function getResults(currentFileType, prompt)
  local config = require('dash.utils.config').config
  local modifiedPrompt = prompt .. ''
  if config.filterWithCurrentFileType and currentFileType and config.fileTypeKeywords[currentFileType] ~= false then
    local keyword = config.fileTypeKeywords[currentFileType] or currentFileType
    modifiedPrompt = keyword .. ':' .. modifiedPrompt
  end
  local result = require('dash.utils.jobs').runSearch(modifiedPrompt)
  local stdout = result.stdout
  local stderr = result.stderr

  if stdout ~= nil then
    local xmlUtils = require('dash.utils.xml')
    local results = xmlUtils.transformItems(xmlUtils.parse(stdout))
    -- special case: for TypeScript, also search JavaScript
    if currentFileType == 'typescript' and config.searchJavascriptWithTypescript then
      results = require('dash.utils.tables').concatArrays(results, getResults('javascript', prompt))
    end

    return results
  end

  if stderr ~= nil then
    print(stderr)
    return {}
  end

  print('Failed to execute Dash.app query')
  return {}
end

local function finderFn(currentFileType)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end

    return getResults(currentFileType, prompt)
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
