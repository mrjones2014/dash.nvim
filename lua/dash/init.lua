local M = {}

local function parseResults(xmlString)
  local xml = require('xml2lua')
  local handler = require('xmlhandler.tree')
  local parser = xml.parser(handler)
  parser:parse(xmlString)
  return handler.root.output or {}
end

local function transformItems(itemsTable)
  local items = {}
  for _, itemsNode in pairs(itemsTable) do
    for _, itemList in pairs(itemsNode) do
      for _, item in pairs(itemList) do
        if item and item.subtitle then
          table.insert(items, item.subtitle[#item.subtitle])
        end
      end
    end
  end
  print(vim.inspect(items))
  return items
end

local function itemNames(items)
  local names = {}
  for _, item in items do
    table.insert(names, item[1])
  end
  return names
end

local function findUidByName(items, name)
  for _, item in items do
    if item[1] == name then
      return item[2]
    end
  end
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
    on_complete = {},
    debounce = 5000,
  })

  pickers
    :new({
      prompt_title = 'Dash',
      finder = finder,
      sorter = sorters.get_generic_fuzzy_sorter(),
      attach_mappings = function(_, map)
        map('i', '<CR>', function(buffnr)
          local entry = require('telescope.actions').get_selected_entry()
          print(vim.inspect(entry))
          --[[ local name = entry[1]
          if not name then
            return
          end
          local uid = findUidByName(name)
          if uid == nil then
            print('No such item with name ' .. name)
            require('telescope.actions').close(buffnr)
            return
          end

          require('dash.utils').openUid(uid) ]]
        end)
        return true
      end,
    })
    :find()
end

function M.search(query)
  picker(query)
end

return M
