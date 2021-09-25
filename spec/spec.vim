set rtp+=.
set rtp+=vendor/plenary.nvim/
set rtp+=vendor/matcher_combinators.lua/
set rtp+=vendor/xml2lua/

runtime plugin/plenary.vim

lua require('plenary.busted')
lua require('matcher_combinators.luassert')
lua require('xml2lua')
