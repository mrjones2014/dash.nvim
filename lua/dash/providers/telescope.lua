local M = {}

local cli_path = require('dash.config').config.dash_app_path .. require('libdash_nvim').DASH_APP_CLI_PATH

local function finder_fn(current_file_type, bang)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end

    local queries = require('dash.query-builder').build_query(current_file_type, prompt, bang)

    if #queries == 0 then
      return {}
    end

    table.insert(queries, 1, cli_path)
    return require('libdash_nvim').query(queries)
  end
end

local function attach_mappings(_, map)
  map('i', '<CR>', function(buffnr)
    local entry = require('telescope.actions.state').get_selected_entry()
    if not entry then
      return
    end
    local libdash = require('libdash_nvim')
    libdash.query({ cli_path, entry.query })
    libdash.open_item(entry.value)
    require('telescope.actions').close(buffnr)
  end)
  return true
end

local function build_picker_title()
  local config = require('dash.config').config
  local filetype = vim.bo.filetype
  local file_type_keywords = config.file_type_keywords[filetype]
  if not file_type_keywords then
    return 'Dash'
  end

  if file_type_keywords == true then
    return 'Dash - ' .. filetype
  end

  if type(file_type_keywords) == 'string' then
    return 'Dash - ' .. file_type_keywords
  end

  if type(file_type_keywords) == 'table' then
    return 'Dash - ' .. (vim.inspect(file_type_keywords)):gsub('\n', '')
  end

  return 'Dash'
end

--- Build a Telescope picker for Dash.app and return it
---@param bang boolean @bang disables filtering by filetype
---@param initial_text string @pre-fill text into the telescope prompt
---@return table @Telescope Picker, has :find() method
function M.build_picker(bang, initial_text)
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
    prompt_title = build_picker_title(),
    finder = finder,
    sorter = Sorter.get_generic_fuzzy_sorter(),
    debounce = require('dash.config').config.debounce,
    attach_mappings = attach_mappings,
    default_text = initial_text,
  })

  return picker
end

return M
