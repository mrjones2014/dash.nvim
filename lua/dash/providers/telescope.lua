local M = {}

local function finder_fn(current_file_type, bang)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end

    return require('libdash_nvim').query(prompt, current_file_type, bang)
  end
end

local function attach_mappings(_, map)
  map('i', '<CR>', function(buffnr)
    local entry = require('telescope.actions.state').get_selected_entry()
    if not entry then
      return
    end
    local libdash = require('libdash_nvim')

    if not entry.is_fallback then
      libdash.query(entry.query, '', true)
      libdash.open_item(entry.value)
    else
      libdash.open_search_engine(entry.value)
    end
    require('telescope.actions').close(buffnr)
  end)
  return true
end

--- Build a Telescope picker for Dash.app and return it
---@param bang boolean @bang disables filtering by filetype
---@param initial_text string @pre-fill text into the telescope prompt
function M.dash(bang, initial_text)
  local Picker = require('telescope.pickers')
  local Finder = require('telescope.finders')
  local Sorter = require('telescope.sorters')
  local finder = Finder.new_dynamic({
    fn = finder_fn(vim.bo.filetype, bang),
    entry_maker = function(entry)
      return entry
    end,
    on_complete = {},
  })

  local picker = Picker:new({
    prompt_title = require('dash.providers').build_picker_title(bang),
    finder = finder,
    sorter = Sorter.get_generic_fuzzy_sorter(),
    debounce = require('libdash_nvim').config.debounce,
    attach_mappings = attach_mappings,
    default_text = initial_text,
  })

  picker:find()
end

return M
