local M = {}

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

  require('telescope._extensions.dash')
  require('telescope').load_extension('dash')

  vim.g.loaded_dash_vim = true
end

return M
