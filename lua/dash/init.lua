local M = {}

---@param bang boolean @bang searches without any filtering
---@param initial_text string @pre-fill text into the telescope picker
function M.search(bang, initial_text)
  local opts = { bang = bang or false, initial_text = initial_text or '' }

  local telescope_installed, _ = pcall(require, 'telescope')
  if telescope_installed then
    return require('dash.providers.telescope').dash(opts)
  end

  local fzf_lua_installed, _ = pcall(require, 'fzf-lua')
  if fzf_lua_installed then
    return require('dash.providers.fzf-lua').dash(opts)
  end

  local snap_installed, _ = pcall(require, 'snap')
  if snap_installed then
    return require('dash.providers.snap').dash(opts)
  end

  vim.api.nvim_err_writeln('Dash.nvim: no supported fuzzy-finder plugins found.')
end

--- see src/config.rs for all config keys
--- https://github.com/mrjones2014/dash.nvim/blob/master/src/config.rs
---@param config table
function M.setup(config)
  require('libdash_nvim').setup(config)
end

return M
