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

local function getMainTextValue(item)
  local value = item.text
  if type(value) ~= 'table' then
    return value
  end

  for _, textValue in pairs(item) do
    if textValue._attr and textValue._attr.type == 'copy' then
      return textValue[1]
    end
  end
  return item.title
end

local function transformSingleItem(item)
  local title = item.title
  local value = getMainTextValue(item)
  if item.subtitle then
    if type(item.subtitle) == 'table' then
      title = title .. ': ' .. item.subtitle[#item.subtitle]
    else
      title = title .. ': ' .. item.subtitle
    end
  end
  return { title, value }
end

function M.transformItems(output)
  local items = {}
  for _, item in pairs(output) do
    if type(item) == 'table' and item.title then
      table.insert(items, transformSingleItem(item))
    end
  end
  return items
end

function M.parse(xmlString)
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

return M
