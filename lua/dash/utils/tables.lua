local M = {}

--- Concatenate two array tables
---@param array1 table
---@param array2 table
---@return table
function M.concat_arrays(array1, array2)
  local result = {}
  for _, value in ipairs(array1) do
    table.insert(result, value)
  end

  for _, value in ipairs(array2) do
    table.insert(result, value)
  end

  return result
end

local function deep_copy_helper(orig, copies)
  copies = copies or {}
  local orig_type = type(orig)
  local copy
  if orig_type == 'table' then
    if copies[orig] then
      copy = copies[orig]
    else
      copy = {}
      copies[orig] = copy
      for origKey, origValue in next, orig, nil do
        copy[deep_copy_helper(origKey, copies)] = deep_copy_helper(origValue, copies)
      end
      setmetatable(copy, deep_copy_helper(getmetatable(orig), copies))
    end
  else -- number, string, boolean, etc
    copy = orig
  end
  return copy
end

-- Create a deep copy of a table
---@param tbl table
---@return table
function M.deepcopy(tbl)
  return deep_copy_helper(tbl)
end

--- Merge two tables together
---@param t1 table
---@param t2 table
---@return table
function M.merge_tables(t1, t2)
  for k, v in pairs(t2) do
    if type(v) == 'table' then
      if type(t1[k] or false) == 'table' then
        M.merge_tables(t1[k] or {}, t2[k] or {})
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
