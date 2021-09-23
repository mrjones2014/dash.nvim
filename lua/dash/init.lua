local M = {}

local function parseResults(xmlString)
  local xml = require('xml2lua')
  local handler = require('xmlhandler.tree')
  local parser = xml.parser(handler)
  parser:parse(xmlString)
  return handler.root.output or {}
end

local function isArray(any)
  return type(any) == 'table' and #any > 0 and next(any, #any) == nil
end

local function transformItems(output)
  local items = {}
  local sourceItems = output.items

  if not sourceItems then
    sourceItems = output
  end

  if not sourceItems then
    print('failed to parse XML')
    return {}
  end

  for _, item in pairs(sourceItems) do
    if not item._attr then
      for _, subitem in pairs(item) do
        if subitem._attr then
          table.insert(items, { subitem.subtitle[#subitem.subtitle], subitem._attr.uid })
        end
      end
    elseif type(item) == 'table' then
      table.insert(items, { item.subtitle[#item.subtitle], item._attr.uid })
    end
  end
  print(#items)
  return items
end

local function picker()
  local pickers = require('telescope.pickers')
  local finders = require('telescope.finders')
  local sorters = require('telescope.sorters')

  local finderFn = function(prompt)
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
