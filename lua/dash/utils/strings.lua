local M = {}

local char_to_hex = function(c)
  return string.format('%%%02X', string.byte(c))
end

--- URL encode the given string
---@param url string @the string to URL encode
---@return string
function M.urlencode(url)
  if url == nil then
    return
  end
  url = url:gsub('\n', '\r\n')
  url = string.gsub(url, '([^%w _ %- . ~])', char_to_hex)
  url = url:gsub(' ', '+')
  return url
end

--- Join an array of strings to a newline-separated string
---@param arr table @the array of strings to join
---@return string
function M.join_list_to_string(arr)
  if not (type(arr) == 'table') then
    return arr
  end

  local str = ''
  for _, val in pairs(arr) do
    str = str .. val .. '\n'
  end
  return M.trim_trailing_newlines(str)
end

--- Trim trailing newlines from a string
---@param str string @the string to trim
---@return string
function M.trim_trailing_newlines(str)
  if str == nil then
    return nil
  end
  local n = #str
  while n > 0 and str:find('^%s', n) do
    n = n - 1
  end
  return str:sub(1, n)
end

return M
