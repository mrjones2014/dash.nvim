if exists('g:loaded_dash_nvim')
  finish
endif

function s:dash_nvim_search(bang, use_word_under_cursor)
  if a:bang == 1
    if a:use_word_under_cursor
      lua require('telescope').extensions.dash.search(true, vim.fn.expand('<cword>'))
      return
    endif

    lua require('telescope').extensions.dash.search(true)
    return
  endif

  if a:use_word_under_cursor
    lua require('telescope').extensions.dash.search(false, vim.fn.expand('<cword>'))
    return
  endif

  lua require('telescope').extensions.dash.search()
endfunction

command! -nargs=0 -bang Dash :call <SID>dash_nvim_search(<bang>0, v:false)
command! -nargs=0 -bang DashWord :call <SID>dash_nvim_search(<bang>0, v:true)

let g:dash_root_dir = expand('<sfile>:p:h:h')

lua require('telescope._extensions.dash')
lua require('telescope').load_extension('dash')

let g:loaded_dash_nvim = 1
