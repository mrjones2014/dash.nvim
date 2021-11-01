local M = {}

local function yield_results(results)
  local snap = require('snap')
  local transformed_results = {}
  for _, item in pairs(results) do
    table.insert(transformed_results, snap.with_metas(item.display, item))
  end
  coroutine.yield(transformed_results)
end

local function build_producer(current_file_type, bang)
  return function(request)
    if request and request.filter and #request.filter > 0 then
      local results = require('libdash_nvim').query(request.filter, current_file_type, bang or false)
      yield_results(results)
    end
  end
end

local function handle_selected(selected)
  print(vim.inspect(selected))
end

function M.dash(bang)
  local current_file_type = vim.bo.filetype
  return require('snap').run({
    producer = require('snap').get('consumer.fzf')(build_producer(current_file_type, bang or false)),
    select = handle_selected,
  })
end

return M
