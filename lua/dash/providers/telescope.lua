local M = {}

local function finder_fn(current_file_type, bang)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end

    return require('libdash_nvim').query({
      search_text = prompt,
      buffer_type = current_file_type,
      ignore_keywords = bang,
    })
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
      libdash.open_item(entry)
    else
      libdash.open_url(entry.value)
    end
    require('telescope.actions').close(buffnr)
  end)
  return true
end

function M.dash(opts)
  local Picker = require('telescope.pickers')
  local Finder = require('telescope.finders')
  local Sorter = require('telescope.sorters')
  local finder = Finder.new_dynamic({
    fn = finder_fn(vim.bo.filetype, opts.bang or false),
    entry_maker = function(entry)
      return entry
    end,
    on_complete = {},
  })

  local picker = Picker:new({
    prompt_title = require('dash.providers').build_picker_title(opts.bang or false),
    finder = finder,
    sorter = Sorter.get_generic_fuzzy_sorter(),
    debounce = require('libdash_nvim').config.debounce,
    attach_mappings = attach_mappings,
    default_text = opts.initial_text or '',
  })

  picker:find()
end

return M
