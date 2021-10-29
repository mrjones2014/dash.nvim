<!-- panvimdoc-ignore-start -->

![Build](https://github.com/mrjones2014/dash.nvim/actions/workflows/lint-check-test.yml/badge.svg) [![Lua](https://img.shields.io/badge/Made%20With-Lua-blue)](https://www.lua.org) [![Rust](https://img.shields.io/badge/Made%20With-Rust-red)](https://www.rust-lang.org)

<!-- panvimdoc-ignore-end -->

# Dash.nvim

Query [Dash.app](https://kapeli.com/dash) within Neovim with a Telescope picker!

<!-- panvimdoc-ignore-start -->

![demo](./images/demo.gif)

The theme used in the recording is [lighthaus.nvim](https://github.com/mrjones2014/lighthaus.nvim).

<!-- panvimdoc-ignore-end -->

Note: Dash is a Mac-only app, so you'll only find this plugin useful on Mac.

## Install

After installing Dash.nvim, you must run `make install`. This can be done through a post-install hook with most plugin managers.

Packer:

```lua
use({
  'mrjones2014/dash.nvim',
  requires = { 'nvim-telescope/telescope.nvim' },
  run = 'make install',
  disable = not vim.fn.has('macunix'),
})
```

Paq:

```lua
require("paq")({
  'nvim-telescope/telescope.nvim';
  {'mrjones2014/dash.nvim', run = 'make install'}
})
```

Vim-Plug:

```VimL
Plug 'nvim-telescope/telescope.nvim'
Plug 'mrjones2014/dash.nvim', { 'do': 'make install' }
```

## Usage

<!-- panvimdoc-ignore-start -->

Run `:h dash` to see these docs in Neovim.

<!-- panvimdoc-ignore-end -->

### Editor Commands

This plugin has two editor commands, `:Dash` and `:DashWord`, each of which accept a bang (`!`). By default, it will
search Dash.app with keywords based on config (see `file_type_keywords` in [configuration](#configuration)). The bang (`!`)
will search without this keyword filtering.

`:Dash [query]` will open the Telescope picker, and if `[query]` is passed, it will pre-fill the prompt with `[query]`.

`:DashWord` will open the Telescope picker and pre-fill the prompt with the word under the cursor.

## Configuration

`dash.nvim` can be configured in your Telescope config. Options and defaults are described below:

```lua
require('telescope').setup({
  extensions = {
    dash = {
      -- configure path to Dash.app if installed somewhere other than /Applications/Dash.app
      dash_app_path = '/Applications/Dash.app',
      -- search engine to fall back to when Dash has no results, must be one of: 'ddg', 'duckduckgo', 'startpage', 'google'
      search_engine = 'ddg',
      -- debounce while typing, in milliseconds
      debounce = 0,
      -- map filetype strings to the keywords you've configured for docsets in Dash
      -- setting to false will disable filtering by filetype for that filetype
      -- filetypes not included in this table will not filter the query by filetype
      -- check lua/dash.config.lua to see all defaults
      -- the values you pass for file_type_keywords are merged with the defaults
      -- to disable filtering for all filetypes,
      -- set file_type_keywords = false
      file_type_keywords = {
        dashboard = false,
        NvimTree = false,
        TelescopePrompt = false,
        terminal = false,
        packer = false,
        -- a table of strings will search on multiple keywords
        javascript = { 'javascript', 'nodejs' },
        typescript = { 'typescript', 'javascript', 'nodejs' },
        typescriptreact = { 'typescript', 'javascript', 'react' },
        javascriptreact = { 'javascript', 'react' },
        -- you can also do a string, for example,
        -- bash = 'sh'
      },
    }
  }
})
```

If you notice an issue with the default `file_type_keywords` or would like a new filetype added, please file an issue or submit a PR!

### Lua API

The public API consists of two main functions.

```lua
-- See lua/dash.config.lua for full DashConfig type definition
-- Also described in configuration section below
---@param config DashConfig
require('dash').setup(config)
```

```lua
---@param bang boolean @bang searches without any filtering
---@param initial_text string @pre-fill text into the telescope picker
require('dash').search(bang, initial_text)
```

See [backend](#Backend) for documentation on the backend data provider.

## Backend

The binaries for the Rust backend can be found under `bin/`, compiled for Mac M1 and Intel architectures.
To build from source, you will need a Rust toolchain, which can be installed from [rustup.rs](https://rustup.rs).
Once this is installed, you should be able to build via `make build`. Then, `make install` will copy the correct
binary into the `lua/` directory so that it is added to Lua's runtimepath.

The Rust backend is exposed as a Lua module. To `require` the module, you will need to have the file `libdash_nvim.so` for your architecture (M1 or Intel)
on your runtimepath, as well as the `deps` directory, which must be in the same directory as the `libdash_nvim.so` shared library file.

### Constants

The Rust backend exports the following constants for use:

- `require('libdash_nvim').DASH_APP_BASE_PATH` => "/Applications/Dash.app"
- `require('libdash_nvim).DASH_APP_CLI_PATH` => "/Contents/Resources/dashAlfredWorkflow"

### `libdash_nvim.query`

This method (`require('libdash_nivm').query`) takes 4 arguments: the search text, the current buffer type,
whether to disable filetype filtering (e.g. command was run with bang, `:Dash!`), and the configuration table.

```lua
local libdash = require('libdash_nvim')
local results = libdash.query(
  'match arms',
  'rust',
  false,
  require('dash.config').config
)
```

The `query` method returns a table with the following properties:

- `value` -- the number value of the item, to be used when selected. Running a query, then opening the URL `dash-workflow-callback://[value]` will open the selected item in Dash.app
- `ordinal` -- a value to sort by, currently this is the same value as `display`
- `display` -- a display value
- `keyword` -- the keyword (if there was one) on the query that returned this result
- `query` -- the full query that returned this result

If no items are returned from querying Dash, it will return a single item with an extra key, `fallback = true`. The table will look something like the following:

```lua
{
  value = 'https://duckduckgo.com/?q=array.prototype.filter',
  ordinal = '1',
  display = 'Search with DuckDuckGo: array.prototype.filter',
  is_fallback = true,
}
```


### `libdash_nvim.open_item`

Takes the `value` property of an item returned from querying Dash and opens it in Dash.

```lua
require('libdash_nvim').open_item(1)
```

**Note:** if running multiple queries, simply opening `dash-workflow-callback://[value]` may not work directly. Opening the URL assumes that
the value being opened was returned by the currently active query in Dash.app. You can work around this by just running the query again with
only the `query` value from the selected item, then calling `require('libdash_nvim).open` with that item's `value`.

### `libdash_nvim.open_search_engine`

Utility method to open a search engine URL when the fallback item is selected.

```lua
require('libdash_nvim').open_search_engine('https://duckduckgo.com/?q=array.prototype.filter')
```

---

## Contributing

### Git Hooks

If you plan on changing Rust code, you will need to install the git hooks via `make install-hooks`.
The git hooks require you have a Rust toolchain installed. You can install a Rust toolchain from
[rustup.rs](https://rustup.rs).

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

Use `snake_case` for everything. Ensure you use [EmmyLua Annotations](https://github.com/sumneko/lua-language-server/wiki/EmmyLua%2DAnnotations)
for any public-facing API, and optionally for non-public functions, if the function is non-trivial or the types are not obvious.
Other than that, running `luacheck` and `stylua` should cover it.
