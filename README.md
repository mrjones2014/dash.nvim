![Build](https://github.com/mrjones2014/dash.nvim/actions/workflows/lint-check-test.yml/badge.svg)

# Dash.nvim

Query [Dash.app](https://kapeli.com/dash) within Neovim with a Telescope picker!

![demo](./images/demo.gif)

Note: Dash is a Mac-only app, so you'll only find this plugin useful on Mac.

## Usage

Show the picker with `:Dash` or `require('dash').search()`

## Install

Using Packer:

```lua
use({ 'mrjones2014/dash.nvim', requires = { 'nvim-telescope/telescope.nvim' } })
```

## Configuration

All options are set by calling `require('dash').setup(config)`. Options and defaults are described below:

```lua
{
  -- configure path to Dash.app if installed somewhere other than /Applications/Dash.app
  dashAppPath = '/Applications/Dash.app',
  -- map filetype strings to the keywords you've configured for docsets in Dash
  -- setting to false will disable filtering by filetype for that filetype
  fileTypeKeywords = {
    dashboard = false,
    NvimTree = false,
    TelescopePrompt = false,
    terminal = false,
    packer = false,
    -- e.g.
    -- javascript = 'js'
  },
  -- disable filtering by current filetype for all filetypes
  filterWithCurrentFileType = true,
  -- by default, searching in a TypeScript file will search both TypeScript and JavaScript docsets,
  -- set to false to disable this behavior
  searchJavascriptWithTypescript = true,
}
```

---

## Contributing

### Running Tests

Tests This uses [busted][busted], [luassert][luassert] (both through
[plenary.nvim][plenary]) and [matcher_combinators][matcher_combinators] to
define tests in `spec/` directory. These dependencies are required only to run
tests, that´s why they are installed as git submodules.

To run tests, run `make test`. This runs tests in Neovim with a minimal profile,
[spec.vim](./spec/spec.vim). This runs Neovim with only this plugin, and the testing dependencies.

If you have [entr(1)](https://eradman.com/entrproject/) installed, you can run the tests in watch mode
using `make watch`.

### Code Style

Use `cameCase` for everything. Other than that, running `luacheck` and `stylua` should cover it.
