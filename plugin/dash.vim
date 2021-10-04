if exists('g:loaded_dash_nvim')
  finish
endif

function s:run_with_bang(bang)
  if a:bang == 1
    lua require('telescope').extensions.dash.search(true)
    return
  endif

  lua require('telescope').extensions.dash.search()
endfunction

command! -nargs=0 -bang Dash :call <SID>run_with_bang(<bang>0)
lua require('telescope._extensions.dash')

let g:loaded_dash_nvim = 1
