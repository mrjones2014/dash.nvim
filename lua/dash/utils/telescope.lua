local M = {}

local function get_results_for_filetype(current_file_type, prompt, bang)
  local config = require('dash.utils.config').config
  local file_type_keywords = config.file_type_keywords[current_file_type]
  if bang == true or not file_type_keywords then
    local stdout, stderr = require('dash.utils.jobs').run_queries({ prompt })

    -- filtering by filetype is disabled
    if stderr then
      print('is it here')
      print(stderr)
      return {}
    end

    if stdout then
      return vim.fn.json_decode(stdout)
    end
  end

  if file_type_keywords == true then
    prompt = current_file_type .. ':' .. prompt
    local stdout, stderr = require('dash.utils.jobs').run_queries({ prompt })

    if stderr then
      print('is it here')
      print(stderr)
      return {}
    end

    if stdout then
      return vim.fn.json_decode(stdout)
    end
  end

  if type(file_type_keywords) == 'table' then
    local queries = {}
    for _, value in ipairs(file_type_keywords) do
      table.insert(queries, value .. ':' .. prompt)
    end

    local stdout, stderr = require('dash.utils.jobs').run_queries(queries)

    if stderr then
      print(stderr)
      return {}
    end

    if stdout then
      return vim.fn.json_decode(stdout)
    end
  end

  return {}
end

local function finder_fn(current_file_type, bang)
  return function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end

    return get_results_for_filetype(current_file_type, prompt, bang)
  end
end

local function attach_mappings(_, map)
  map('i', '<CR>', function(buffnr)
    local entry = require('telescope.actions').get_selected_entry()
    if not entry then
      return
    end
    local query = entry.value
    if entry.keyword ~= nil and type(entry.keyword) == 'string' then
      query = entry.keyword .. ':' .. query
    end
    local jobs = require('dash.utils.jobs')
    jobs.run_queries({ query })
    jobs.open_query(query)
    require('telescope.actions').close(buffnr)
  end)
  return true
end

local function build_picker_title()
  local config = require('dash.utils.config').config
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
---@return table @Telescope Picker, has :find() method
function M.build_picker(bang)
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
    debounce = require('dash.utils.config').config.debounce,
    attach_mappings = attach_mappings,
  })

  return picker
end

return M
