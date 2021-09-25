local M = {}

local char_to_hex = function(c)
  return string.format('%%%02X', string.byte(c))
end

function M.urlencode(url)
  if url == nil then
    return
  end
  url = url:gsub('\n', '\r\n')
  url = string.gsub(url, '([^%w _ %- . ~])', char_to_hex)
  url = url:gsub(' ', '+')
  return url
end

function M.joinListToString(output)
  if not (type(output) == 'table') then
    return output
  end

  local str = ''
  for _, val in pairs(output) do
    str = str .. val .. '\n'
  end
  return M.trimTrailingNewlines(str)
end

function M.trimTrailingNewlines(str)
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
