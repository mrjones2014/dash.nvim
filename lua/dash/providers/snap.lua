local M = {}

local function yield_results(results)
  local snap = require('snap')
  for _, item in pairs(results) do
    coroutine.yield({ snap.with_metas(item.display, item) })
  end
end

local function build_producer(current_file_type, bang)
  return function(request)
    if not request or request.canceled() then
      print('wtf')
      coroutine.yield(nil)
    elseif request.filter and #request.filter > 0 then
      local results = require('libdash_nvim').query({
        search_text = request.filter,
        buffer_type = current_file_type,
        ignore_keywords = bang or false,
      })
      yield_results(results)
    else
      coroutine.yield({})
    end
  end
end

local function handle_selected(selected)
  local libdash = require('libdash_nvim')
  if selected.is_fallback then
    libdash.open_url(selected.value)
  else
    libdash.open_item(selected)
  end
end

function M.dash(bang)
  local current_file_type = vim.bo.filetype
  return require('snap').run({
    producer = build_producer(current_file_type, bang or false),
    select = handle_selected,
    prompt = 'Dash>',
  })
end

return M
