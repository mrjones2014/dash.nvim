if exists('g:loaded_dash_nvim')
  finish
endif

command! -nargs=0 Dash lua require('telescope').extensions.dash.search()
lua require('telescope._extensions.dash')

let g:loaded_dash_nvim = 1
