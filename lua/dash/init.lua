local M = {}

-- lazy-constructed Telescope picker
local picker = nil

local function getPicker()
  if picker ~= nil then
    return picker
  end

  local Picker = require('telescope.pickers')
  local Finder = require('telescope.finders')
  local Sorter = require('telescope.sorters')
  local finderUtils = require('dash.utils.telescope')
  local finder = Finder.new_dynamic({
    fn = finderUtils.finderFn,
    entry_maker = finderUtils.entryMaker,
    on_complete = {},
  })

  picker = Picker:new({
    prompt_title = 'Dash',
    finder = finder,
    sorter = Sorter.get_generic_fuzzy_sorter(),
    attach_mappings = finderUtils.attachMappings,
  })

  return picker
end

function M.search()
  getPicker():find()
end

function M.setup(config)
  require('dash.utils.config').setup(config)
end

return M
