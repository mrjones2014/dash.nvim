if exists('g:loaded_dash_nvim')
  finish
endif

function s:dash_nvim_search(bang, initial_search)
  let l:initial_search = type(a:initial_search) == v:t_string && len(a:initial_search) > 0 ? '"' . a:initial_search . '"' : 'nil'
  let l:bang = a:bang == 1 ? 'true' : 'false'
  let l:dash_lua_call = 'require("dash").search(' . l:bang . ', ' . l:initial_search . ')'
  call luaeval(l:dash_lua_call)
endfunction

command! -nargs=* -bang Dash :call <SID>dash_nvim_search(<bang>0, <q-args>)
command! -nargs=0 -bang DashWord :call <SID>dash_nvim_search(<bang>0, expand('<cword>'))

lua require('dash.startup').init()
