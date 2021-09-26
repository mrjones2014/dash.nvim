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

local function deepCopyHelper(orig, copies)
  copies = copies or {}
  local origType = type(orig)
  local copy
  if origType == 'table' then
    if copies[orig] then
      copy = copies[orig]
    else
      copy = {}
      copies[orig] = copy
      for origKey, origValue in next, orig, nil do
        copy[deepCopyHelper(origKey, copies)] = deepCopyHelper(origValue, copies)
      end
      setmetatable(copy, deepCopyHelper(getmetatable(orig), copies))
    end
  else -- number, string, boolean, etc
    copy = orig
  end
  return copy
end

function M.deepcopy(tbl)
  return deepCopyHelper(tbl)
end

function M.mergeTables(t1, t2)
  for k, v in pairs(t2) do
    if type(v) == 'table' then
      if type(t1[k] or false) == 'table' then
        M.mergeTables(t1[k] or {}, t2[k] or {})
      else
        t1[k] = v
      end
    else
      t1[k] = v
    end
  end
  return t1
end

return M
