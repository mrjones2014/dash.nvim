local M = {}

local function build_cli_path(dash_app_path)
  -- gsub to remove trailing slash, if there is one, because we're adding one
  return (dash_app_path:gsub('(.)%/$', '%1')) .. require('libdash_nvim').DASH_APP_CLI_PATH
end

local function get_keys(tbl)
  local keyset = {}
  for k, _ in pairs(tbl) do
    keyset[#keyset + 1] = k
  end
  return keyset
end

local function check_fuzzy_finder()
  local health_ok = vim.fn['health#report_ok']
  local health_error = vim.fn['health#report_error']

  local telescope_ok, _ = pcall(require, 'telescope')
  if telescope_ok then
    health_ok("Fuzzy finder plugin 'telescope' is installed.")
  end

  local fzf_lua_ok, _ = pcall(require, 'fzf-lua')
  if fzf_lua_ok then
    health_ok("Fuzzy finder plugin 'fzf-lua' is installed.")
  end

  local snap_ok, _ = pcall(require, 'snap')
  if snap_ok then
    health_ok("Fuzzy finder plugin 'snap' is installed.")
  end

  if not telescope_ok and not fzf_lua_ok and not snap_ok then
    health_error('No supported fuzzy finder plugins are installed.')
  end
end

function M.check()
  -- health_start begins a new section
  local health_start = vim.fn['health#report_start']
  local health_ok = vim.fn['health#report_ok']
  local health_error = vim.fn['health#report_error']
  local health_info = vim.fn['health#report_info']

  health_start('Dependencies')
  -- check whether we can require one of the supported fuzzy-finders successfully successfully
  check_fuzzy_finder()

  -- ensure config is setup
  require('telescope').load_extension('dash')

  health_start('Configuration')
  local config = require('libdash_nvim').config
  local dash_app_path = config.dash_app_path
  local cli_path = build_cli_path(dash_app_path)
  -- check if Dash.app CLI is executable at configured path
  if vim.fn.executable(cli_path) ~= 0 then
    health_ok('Dash.app found and executable at configured path: ' .. dash_app_path)
  else
    health_error('Dash.app not found at configured path: ' .. dash_app_path)

    -- if not executable at configured path, check if executable at default path
    local default_path = require('libdash_nvim').default_config.dash_app_path
    if vim.fn.executable(build_cli_path(default_path)) == 0 then
      health_info('Dash.app found and executable at default path: ' .. default_path)
    end
  end

  -- check status of filtering by filetype
  if not config.file_type_keywords or #(get_keys(config.file_type_keywords)) == 0 then
    health_info(
      'Filetype filtering is disabled.\nconfig.file_type_keywords = ' .. vim.inspect(config.file_type_keywords)
    )
  else
    health_info(
      'Filetype filtering is enabled.\nconfig.file_type_keywords = ' .. vim.inspect(config.file_type_keywords)
    )
  end
end

return M
