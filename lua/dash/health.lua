local M = {}

local function build_cli_path(dash_app_path)
  -- gsub to remove trailing slash, if there is one, because we're adding one
  return (dash_app_path:gsub('(.)%/$', '%1')) .. '/Contents/Resources/dashAlfredWorkflow'
end

function M.check()
  local health_start = vim.fn['health#report_start']
  local health_ok = vim.fn['health#report_ok']
  local health_error = vim.fn['health#report_error']
  local health_info = vim.fn['health#report_info']

  health_start('Dependencies')
  local telescope_ok, _ = pcall(require, 'telescope')
  if not telescope_ok then
    health_error("Module 'telescope' not found.")
  else
    health_ok("Module 'telescope' installed.")
  end

  local plenary_ok, _ = pcall(require, 'plenary')
  if not plenary_ok then
    health_error("Module 'plenary' not found.")
  else
    health_ok("Module 'plenary' installed.")
  end

  -- ensure config is setup
  require('telescope').load_extension('dash')

  health_start('Configuration')
  local config = require('dash.utils.config').config
  local dash_app_path = config.dash_app_path
  local cli_path = build_cli_path(dash_app_path)
  if vim.fn.executable(cli_path) ~= 0 then
    health_ok('Dash.app found and executable at configured path: ' .. dash_app_path)
  else
    health_error('Dash.app not found at configured path: ' .. dash_app_path)

    if vim.fn.executable(build_cli_path('/Applications/Dash.app')) == 0 then
      health_info('Dash.app found and executable at default path: /Applications/Dash.app')
    end
  end

  if not config.file_type_keywords or #(require('dash.utils.tables').get_keys(config.file_type_keywords)) == 0 then
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
