<!-- panvimdoc-ignore-start -->

![Build](https://github.com/mrjones2014/dash.nvim/actions/workflows/lint-check-test.yml/badge.svg) [![Rust](https://img.shields.io/badge/Made%20With-Rust-red)](https://www.rust-lang.org) [![Lua](https://img.shields.io/badge/Made%20With-Lua-blue)](https://www.lua.org)

<!-- panvimdoc-ignore-end -->

# Dash.nvim

Query [Dash.app](https://kapeli.com/dash) within Neovim with your fuzzy finder!

<!-- panvimdoc-ignore-start -->

![demo](./images/demo.gif)

The theme used in the recording is [lighthaus.nvim](https://github.com/mrjones2014/lighthaus.nvim).

<!-- panvimdoc-ignore-end -->

Note: Dash is a Mac-only app, so you'll only find this plugin useful on Mac.

## Install

This plugin must be loaded *after* your fuzzy finder plugin of choice. Currently supported fuzzy finder plugins are:

- [telescope.nvim](https://github.com/nvim-telescope/telescope.nvim)
- [fzf-lua](https://github.com/ibhagwan/fzf-lua)
- [snap](https://github.com/camspiers/snap)

After installing Dash.nvim, you must run `make install`. This can be done through a post-install hook with most plugin managers.

Packer:

```lua
use({
  'mrjones2014/dash.nvim',
  run = 'make install',
})
```

Paq:

```lua
require("paq")({
  { 'mrjones2014/dash.nvim', run = 'make install' };
})
```

Vim-Plug:

```VimL
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

`:Dash [query]` will open the fuzzy finder, and if `[query]` is passed, it will pre-fill the prompt with `[query]`. This
is essentially an alias to `:lua require('dash').search(bang, [query])`.

`:DashWord` will open the fuzzy finder and pre-fill the prompt with the word under the cursor. This is essentially
an alias to `:lua require('dash').search(bang, <cword>)`.

The Lua function `require('dash').search()` will bind to the first supported fuzzy finder plugin it detects. Having multiple fuzzy finder
plugins installed will result in undefined behavior. You can use a specific fuzzy finder's provider directly via
`:lua require('dash.providers.telescope').dash({ bang = false, initial_text = '' })`, for example.

If using Telescope, you can also run `:Telescope dash search` or `:Telescope dash search_no_filter`.

If using fzf-lua, you can also run `:FzfLua dash` or `:lua require('fzf-lua').dash({ bang = false, initial_text = '' })`.

If using Snap, you can also run `:lua require('dash.providers.snap').dash({ bang = false, initial_text = '' })`.

## Configuration

### Configuration Table Structure

```lua
{
  -- configure path to Dash.app if installed somewhere other than /Applications/Dash.app
  dash_app_path = '/Applications/Dash.app',
  -- search engine to fall back to when Dash has no results, must be one of: 'ddg', 'duckduckgo', 'startpage', 'google'
  search_engine = 'ddg',
  -- debounce while typing, in milliseconds
  debounce = 0,
  -- map filetype strings to the keywords you've configured for docsets in Dash
  -- setting to false will disable filtering by filetype for that filetype
  -- filetypes not included in this table will not filter the query by filetype
  -- check src/lua_bindings/dash_config_binding.rs to see all defaults
  -- the values you pass for file_type_keywords are merged with the defaults
  -- to disable filtering for all filetypes,
  -- set file_type_keywords = false
  file_type_keywords = {
    dashboard = false,
    NvimTree = false,
    TelescopePrompt = false,
    terminal = false,
    packer = false,
    fzf = false,
    -- a table of strings will search on multiple keywords
    javascript = { 'javascript', 'nodejs' },
    typescript = { 'typescript', 'javascript', 'nodejs' },
    typescriptreact = { 'typescript', 'javascript', 'react' },
    javascriptreact = { 'javascript', 'react' },
    -- you can also do a string, for example,
    -- sh = 'bash'
  },
}
```

If you notice an issue with the default config or would like a new filetype added, please file an issue or submit a PR!

### With Telescope

```lua
require('telescope').setup({
  extensions = {
    dash = {
      -- your config here
    }
  }
})
```

### With fzf-lua or Snap

```lua
require('dash').setup({
  -- your config here
})
```

### Lua API

The public API consists of two main functions.

```lua
-- See src/lua_bindings/dash_config_binding.rs for available config keys
-- Also described in configuration section below
---@param config
require('dash').setup(config)
```

```lua
--- This will bind to the first fuzzy finder it finds to be available,
--- checked in order: telescope, fzf-lua
---@param bang boolean @bang searches without any filtering
---@param initial_text string @pre-fill text into the finder prompt
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

### `libdash_nvim.config` (table)

This table stores the internal configuration. You can access it via `require('libdash_nvim').config`.
See `src/lua_bindings/dash_config_binding.rs` or [configuration](#configuration) above for configuration keys.

### `libdash_nvim.default_config` (table)

This table stores the *default* configuration. **You should not modify this table, treat it as read-only.** This is mainly
to help with merging your custom config with the default config, but can be useful for debugging purposes. For example:

```VimL
:lua print(vim.inspect(require('libdash_nvim').default_config))
```

### `libdash_nvim.setup` (function)

This method is used to set the internal configuration of the backend. It takes a table, which will be
**merged with the default configuration**. See `src/lua_bindings/dash_config_binding.rs` or [configuration](#configuration) above
for configuration keys.

```lua
require('libdash_nvim').setup({
  -- your custom configuration here
})
```

### `libdash_nvim.query` (function)

This method takes a table as its argument. The table should have the following keys:

- `search_text` - the search text entered by the user
- `buffer_type` - the current buffer type, this will be used to determine filter keywords from config
- `ignore_keywords` - disables filtering by keywords if true (e.g. if run with bang, `:Dash!` or `:DashWord!`)

```lua
local libdash = require('libdash_nvim')
local results = libdash.query({
  search_text = 'match arms',
  buffer_type = 'rust',
  ignore_keywords = false
})
```

The `query` method returns a table list of tables (a Rust `Vec<DashItem>` serialized to a Lua table, see `src/dash_item.rs`)
with the following properties:

- `value` - the number value of the item, to be used when selected
- `ordinal` - a value to sort by, currently this is the same value as `display`
- `display` - a display value
- `keyword` - the keyword (if there was one) on the query that returned this result
- `query` - the full query that returned this result
- `is_fallback` - indicates whether the item represents a search engine fallback and should be handled as such

If no items are returned from querying Dash, it will return a single item with an extra key, `is_fallback = true`.
The table will look something like the following:

```lua
{
  value = 'https://duckduckgo.com/?q=rust match arms',
  ordinal = '1',
  display = 'Search with DuckDuckGo: rust match arms',
  keyword = 'rust',
  query = 'rust:match arms',
  is_fallback = true,
}
```

### `libdash_nvim.open_item` (function)

Takes an item returned from querying Dash via the `require('libdash_nvim').query` function and opens it in Dash.

```lua
local libdash = require('libdash_nvim')
local results = libdash.query({
  search_text = 'match arms',
  buffer_type = 'rust',
  ignore_keywords = false
})
local selected = results[1]
require('libdash_nvim').open_item(selected)
```

### `libdash_nvim.open_url` (function)

Simply takes a URL string and opens it in the default browser/handler for the URL protocol. This is
used for both opening the search engine fallback via an HTTPS URL, as well as opening the selected
`DashItem` in Dash.app via the `dash-workflow-callback://` URL protocol.

```lua
require('libdash_nvim').open_url('https://duckduckgo.com/?q=array.prototype.filter')
require('libdash_nvim').open_url('dash-workflow-callback://5')
```

---

## Contributing

### Git Hooks

If you plan on changing Rust code, you will need to install the git hooks via `make install-hooks`.
The git hooks require you have a Rust toolchain installed. You can install a Rust toolchain from
[rustup.rs](https://rustup.rs).

### Developing Locally

The best way to develop and test locally is to install the plugin from a locally cloned repository.
If you're using Packer, you can just do:

```lua
use({
  '~/git/dash.nvim', -- or whatever your local path is
  run = 'make install',
})
```

Otherwise you can add it manually via:

```lua
vim.opt.runtimepath:append('~/git/dash.nvim') -- or whatever your local path is
```

There is also a `make dev` task which will set the `$DASH_NVIM_DEV` environment variable and open `nvim` for you.
When the `$DASH_NVIM_DEV` environment variable is set, there will be an extra command available, `:DashDevReload`.
This will reload Telescope, fzf-lua, and Snap (whichever ones you have installed), as well as the `dash` and `libdash_nvim`
Lua modules.

### Running Tests

You can run all tests (both Rust and Lua) with `make test`.

#### Lua Tests

This uses [busted](https://github.com/Olivine-Labs/busted), [luassert](https://github.com/Olivine-Labs/luassert) (both through
[plenary.nvim](https://github.com/nvim-lua/plenary.nvim)) and [matcher_combinators](https://github.com/m00qek/matcher_combinators.lua) to
define tests in `spec/` directory. These dependencies are required only to run
tests, that's why they are installed as git submodules.

To run Lua tests, run `make test-lua`. This runs tests in Neovim with a minimal profile,
[spec.vim](./spec/spec.vim). This runs Neovim with only this plugin, and the testing dependencies.

If you have [entr(1)](https://eradman.com/entrproject/) installed, you can run the tests in watch mode
using `make watch`.

#### Rust Tests

Rust tests use built-in Rust assertions and test modules. To run Rust tests, run `make test-rust`.

### Code Style

Use `snake_case` for everything. All Lua code should be checked and formatted with `luacheck` and `stylua`. Only
presentation-layer code (such as providers for various fuzzy finder plugins) should be in the Lua code, any core
functionality most likely belongs in the Rust backend.

All Rust code should be checked and formatted using [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer),
and linted using [clippy](https://github.com/rust-lang/rust-clippy), which can be run via `make lint-rust`.
