if exists('g:loaded_dash_nvim')
  finish
endif
let g:loaded_dash_nvim = 1

command! -nargs=0 Dash lua require('dash').search()
