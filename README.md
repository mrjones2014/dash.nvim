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
  -- filetypes not included in this table will not filter the query by filetype
  -- check lua/dash/utils/config.lua to see all defaults
  fileTypeKeywords = {
    dashboard = false,
    NvimTree = false,
    TelescopePrompt = false,
    terminal = false,
    packer = false,
    -- a table of strings will search on multiple keywords
    typescript = { 'typescript', 'javascript' },
    typescriptreact = { 'typescript', 'javascript', 'react' },
    javascriptreact = { 'javascript', 'react' },
    -- you can also do a string, for example,
    -- bash = 'sh'
  },
}
```

If you notice an issue with the default `fileTypeKeywords` or would like a new filetype added, please file an issue or submit a PR!

---

## Contributing

### Running Tests

This uses [busted](https://github.com/Olivine-Labs/busted), [luassert](https://github.com/Olivine-Labs/luassert) (both through
[plenary.nvim](https://github.com/nvim-lua/plenary.nvim)) and [matcher_combinators](https://github.com/m00qek/matcher_combinators.lua) to
define tests in `spec/` directory. These dependencies are required only to run
tests, that's why they are installed as git submodules.

To run tests, run `make test`. This runs tests in Neovim with a minimal profile,
[spec.vim](./spec/spec.vim). This runs Neovim with only this plugin, and the testing dependencies.

If you have [entr(1)](https://eradman.com/entrproject/) installed, you can run the tests in watch mode
using `make watch`.

### Code Style

Use `camelCase` for everything. Other than that, running `luacheck` and `stylua` should cover it.
