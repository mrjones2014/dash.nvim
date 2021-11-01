local M = {}

function M.build_picker_title(bang)
  local config = require('libdash_nvim').config
  local filetype = vim.bo.filetype
  local file_type_keywords = config.file_type_keywords[filetype]
  if bang or not file_type_keywords then
    return 'Dash'
  end

  if file_type_keywords == true then
    return 'Dash - filtering by: ' .. filetype
  end

  if type(file_type_keywords) == 'string' then
    return 'Dash - filtering by: ' .. file_type_keywords
  end

  if type(file_type_keywords) == 'table' then
    return 'Dash - filtering by: ' .. (vim.inspect(file_type_keywords)):gsub('\n', '')
  end

  return 'Dash'
end

return M
