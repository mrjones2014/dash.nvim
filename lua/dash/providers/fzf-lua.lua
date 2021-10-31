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

function M.setup()
  local last_query = ''
  local raw_act = require('fzf.actions').raw_action(function(args)
    print(vim.inspect(args))
    last_query = args[1]
  end)

  local opts = {
    prompt = 'Dash> ',
    fzf_fn = function(add_item)
      local results = require('libdash_nvim').query('match', 'rust', false)
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
    fzf_opts = {
      ['--query'] = vim.fn.shellescape(last_query),
    },
    _fzf_cli_args = ('--bind=change:execute-silent:%s'):format(vim.fn.shellescape(raw_act)),
  }

  coroutine.wrap(function()
    local selected = require('fzf-lua.core').fzf_files(opts)
    if not selected then
      return
    end
    require('fzf-lua.actions').act(handle_selected, selected, opts)
  end)()
end

return M
