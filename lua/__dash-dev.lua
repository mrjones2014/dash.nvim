if not os.getenv('DASH_NVIM_DEV') then
  return
end

local M = {}

function M.reload_dash()
  local reload = require('plenary.reload').reload_module
  local telescope_ok, _ = pcall(require, 'telescope')
  if telescope_ok then
    reload('telescope')
  end

  local fzf_lua_ok, _ = pcall(require, 'fzf-lua')
  if fzf_lua_ok then
    reload('fzf-lua')
  end

  local snap_ok, _ = pcall(require, 'snap')
  if snap_ok then
    reload('snap')
  end

  reload('dash')
  reload('libdash_nvim')
end

return M
