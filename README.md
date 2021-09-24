# Dash.nvim

Query Dash.app within Neovim with a Telescope picker!

![demo](./images/demo.gif)

## Install

Using Packer:

```lua
use({ 'mrjones2014/dash.nvim', requires = { 'nvim-telescope/telescope.nvim', 'nvim-lua/plenary.nvim' }, rocks = { 'xml2lua' } })
```

## Usage

Show the picker with `:Dash` or `require('dash').search()`
