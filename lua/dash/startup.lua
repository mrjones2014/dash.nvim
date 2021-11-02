local M = {}

local function load_telescope_extension()
  local ok, telescope = pcall(require, 'telescope')
  if ok then
    telescope.load_extension('dash')
  end
end

local function load_fzf_lua_extension()
  local ok, fzf_lua = pcall(require, 'fzf-lua')
  if ok then
    fzf_lua.dash = require('dash.providers.fzf-lua').dash
  end
end

function M.init()
  -- check if `make install` was run
  local ok, libdash = pcall(require, 'libdash_nvim')
  if not ok or libdash == nil then
    print(
      'module "libdash_nvim" not found, did you set up Dash.nvim with `make install` as a post-install hook?'
        .. ' See :h dash-install'
    )
    return
  end

  load_telescope_extension()
  load_fzf_lua_extension()

  vim.g.loaded_dash_vim = true
end

return M
