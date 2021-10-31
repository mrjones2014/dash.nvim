local M = {}

local function handle_selected(selected)
  local libdash = require('libdash_nvim')
  if selected.is_fallback then
    libdash.open_search_engine(selected.value)
  else
    libdash.query(selected.query, '', true)
    libdash.open_item(selected.value)
  end
end

local buffer_to_string = function()
  local content = vim.api.nvim_buf_get_lines(0, 0, vim.api.nvim_buf_line_count(0), false)
  return table.concat(content, '\n')
end

function M.setup()
  local opts = {
    prompt = 'Dash> ',
    fzf_fn = function(add_item)
      local results = require('libdash_nvim').query('match', 'rust', false)
      print(buffer_to_string())
      for _, item in pairs(results) do
        setmetatable(item, {
          __tostring = function(self)
            return self.display
          end,
        })
        add_item(item)
      end
      add_item(nil)
    end,
  }

  coroutine.wrap(function()
    local selected = require('fzf-lua.core').fzf(opts, opts.fzf_fn)
    print(vim.inspect(selected))
    if not selected then
      return
    end
    require('fzf-lua.actions').act(handle_selected, selected, opts)
  end)()
end

return M
