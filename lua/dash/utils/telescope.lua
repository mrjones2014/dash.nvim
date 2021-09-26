local M = {}

local function sortByCurrentFiletype(currentFileType)
  currentFileType = string.lower(currentFileType or '')
  return function(left, right)
    local leftSubtitle = string.lower(left.meta.subtitle or '')
    local rightSubtitle = string.lower(right.meta.subtitle or '')

    -- if left contains it
    if string.find(leftSubtitle, currentFileType) then
      -- and right also contains it, no movement
      if string.find(rightSubtitle, currentFileType) then
        return false
      end

      -- if left contains it and right does not, move left up
      return true
    end

    -- if right contains it and left does not, move right up
    if string.find(rightSubtitle, currentFileType) then
      return false
    end

    -- special case: for TypeScript, also check JavaScript
    if currentFileType == 'typescript' then
      -- if left contains it
      if string.find(leftSubtitle, 'javascript') then
        -- and right also contains it, no movement
        if string.find(rightSubtitle, 'javascript') then
          return false
        end

        -- if left contains it and right does not, move left up
        return true
      end

      -- if right contains it and left does not, move right up
      if string.find(rightSubtitle, 'javascript') then
        return false
      end
    end

    return false
  end
end

local function finderFn(currentFileType)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end
    local result = require('dash.utils.jobs').runSearch(prompt)
    local stdout = result.stdout
    local stderr = result.stderr

    if stdout ~= nil then
      local xmlUtils = require('dash.utils.xml')
      local results = xmlUtils.transformItems(xmlUtils.parse(stdout))
      table.sort(results, sortByCurrentFiletype(currentFileType))
      return results
    end

    if stderr ~= nil then
      print(stderr)
      return {}
    end

    print('Failed to execute Dash.app query')
    return {}
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

  picker = Picker:new({
    prompt_title = 'Dash',
    finder = finder,
    sorter = Sorter.get_generic_fuzzy_sorter(),
    attach_mappings = attachMappings,
  })

  return picker
end

return M
