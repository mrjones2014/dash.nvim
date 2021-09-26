local M = {}

function M.concatArrays(array1, array2)
  local result = {}
  for _, value in ipairs(array1) do
    table.insert(result, value)
  end

  for _, value in ipairs(array2) do
    table.insert(result, value)
  end

  return result
end

function M.cloneTableByValue(obj, seen)
  if type(obj) ~= 'table' then
    return obj
  end

  if seen and seen[obj] then
    return seen[obj]
  end

  local localSeen = seen or {}
  local resultingTable = setmetatable({}, getmetatable(obj))
  localSeen[obj] = resultingTable

  for key, value in pairs(obj) do
    resultingTable[M.cloneTableByValue(key, localSeen)] = M.cloneTableByValue(value, localSeen)
  end

  return resultingTable
end

function M.mergeTables(table1, table2)
  local resultingTable = M.cloneTableByValue(table1)

  for key, value in pairs(table2) do
    if (type(value) == 'table') and (type(resultingTable[key] or false) == 'table') then
      M.mergeTables(resultingTable[key], table2[key])
    else
      resultingTable[key] = value
    end
  end

  return resultingTable
end

return M
