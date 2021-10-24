local M = {}

function M.build_query(current_file_type, prompt, bang)
  local config = require('dash.config').config
  local file_type_keywords = config.file_type_keywords[current_file_type]
  if bang == true or not file_type_keywords then
    return { prompt }
  end

  if file_type_keywords == true then
    return { current_file_type .. ':' .. prompt }
  end

  if type(file_type_keywords) == 'string' then
    return { file_type_keywords .. ':' .. prompt }
  end

  if type(file_type_keywords) == 'table' then
    local queries = {}
    for _, value in ipairs(file_type_keywords) do
      table.insert(queries, value .. ':' .. prompt)
    end

    return queries
  end

  return {}
end

return M
