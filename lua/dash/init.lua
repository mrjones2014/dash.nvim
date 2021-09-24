local M = {}

local function flatten(items)
  local flattened = {}
  for _, item in pairs(items) do
    for _, subitem in pairs(item) do
      table.insert(flattened, subitem)
    end
  end
  return flattened
end

local function parseResults(xmlString)
  local xml = require('xml2lua')
  local handler = require('xmlhandler.tree'):new()
  local parser = xml.parser(handler)
  parser:parse(xmlString)
  if handler.root.output and handler.root.output.items then
    if handler.root.output.items.item and handler.root.output.items.item.title then
      return { handler.root.output.items.item }
    end
    return flatten(handler.root.output.items)
  end
  return {}
end

local function transformSingleItem(item)
  local title = item.title
  local value = item._attr.uid
  if item.subtitle then
    if type(item.subtitle) == 'table' then
      title = title .. ': ' .. item.subtitle[#item.subtitle]
    else
      title = title .. ': ' .. item.subtitle
    end
  end
  return { title, value }
end

local function transformItems(output)
  local items = {}
  for _, item in pairs(output) do
    if type(item) == 'table' and item.title then
      table.insert(items, transformSingleItem(item))
    end
  end
  return items
end

local function picker()
  local pickers = require('telescope.pickers')
  local finders = require('telescope.finders')
  local sorters = require('telescope.sorters')

  local finderFn = function(prompt)
    if not prompt or #prompt == 0 then
      return {}
    end
    local utils = require('dash.utils')
    local result = utils.runSearch(prompt)
    local stdout = result.stdout
    local stderr = result.stderr

    if stdout ~= nil then
      return transformItems(parseResults(stdout))
    end

    if stderr ~= nil then
      print(stderr)
      return {}
    end

    print('something went wrong')
    return {}
  end

  local finder = finders.new_dynamic({
    fn = finderFn,
    entry_maker = function(entry)
      return {
        value = entry[2],
        display = entry[1],
        ordinal = entry[1],
      }
    end,
    on_complete = {},
  })

  pickers
    :new({
      prompt_title = 'Dash',
      finder = finder,
      sorter = sorters.get_generic_fuzzy_sorter(),
      attach_mappings = function(_, map)
        map('i', '<CR>', function(buffnr)
          local entry = require('telescope.actions').get_selected_entry()
          if not entry then
            return
          end
          local utils = require('dash.utils')
          utils.openUid(entry.value)
          require('telescope.actions').close(buffnr)
        end)
        return true
      end,
    })
    :find()
end

function M.test(query)
  local utils = require('dash.utils')
  local result = utils.runSearch(query)
  local stdout = result.stdout
  local stderr = result.stderr

  if stdout ~= nil then
    return transformItems(parseResults(stdout))
  end

  if stderr ~= nil then
    print(stderr)
  end
end

function M.search()
  picker()
end

return M
