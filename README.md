# Dash.nvim

Query [Dash.app](https://kapeli.com/dash) within Neovim with a Telescope picker!

![demo](./images/demo.gif)

Note: Dash is a Mac-only app, so you'll only find this plugin useful on Mac.

## Install

Using Packer:

```lua
use({ 'mrjones2014/dash.nvim', requires = { 'nvim-telescope/telescope.nvim', 'nvim-lua/plenary.nvim' }, rocks = { 'xml2lua' } })
```

## Usage

Show the picker with `:Dash` or `require('dash').search()`
