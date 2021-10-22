# Rust Backend

The Rust backend is a command-line interface. A pre-built binary can be found at `bin/dash-nvim`.

To build from source, you will need a Rust toolchain, which can be installed from [rustup.rs](https://rustup.rs).
Once this is installed, you should be able to build via `make build-rust`.

## API

The Rust backend is exposed as a Lua module. To `require` the module, you will need to have the file `libdash_nvim.so` for your architecture (M1 or Intel)
on your runtimepath, as well as the `deps` directory, which must be in the same directory as the `libdash_nvim.so` shared library file.

The Lua module exports one method, `query`, that takes a list of strings. The first item must be the path to the Dash.app CLI, e.g. `/Applications/Dash.app/Contents/Resources/dashAlfredWorkflow`.

Example:

```lua
local results = require('libdash_nvim').query({
  '/Applications/Dash.app/Contents/Resources/dashAlfredWorkflow',
  'javascript:array.prototype.filter',
  'typescript:array.prototype.filter',
})
```

The `query` method returns a table with the following properties:

- `value` -- the number value of the item, to be used when selected. Running a query, then opening the URL `dash-workflow-callback://[value]` will open the selected item in Dash.app
- `ordinal` -- a value to sort by, currently this is the same value as `display`
- `display` -- a display value
- `keyword` -- the keyword (if there was one) on the query that returned this result
- `query` -- the full query that returned this result

**Note:** if running multiple queries, simply opening `dash-workflow-callback://[value]` may not work directly. Opening the URL assumes that
the value being opened was returned by the currently active query in Dash.app. You can work around this by just running the CLI again with
only the `query` value from the selected item, then opening `dash-workflow-callback://[value]`.
