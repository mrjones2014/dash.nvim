# Rust Backend

The Rust backend is a command-line interface. A pre-built binary can be found at `bin/dash-nvim`.

To build from source, you will need a Rust toolchain, which can be installed from [rustup.rs](https://rustup.rs).
Once this is installed, you should be able to build via `make build-rust`.

## Arguments

There is one named argument, and one positional argument. The `-c` argument can be
used to specify the install location of Dash.app if using a non-standard install location.
If not passed, it defaults to `/Applications/Dash.app`.

The positional argument is the queries to be run. You can pass multiple queries at once. They
must be quoted if the query contains spaces.

### Examples

`bin/dash-nvim -c /path/to/Dash.app "typescript:array.prototype.filter" "javascript:array.prototype.filter" "nodejs:array.prototype.filter"`

`bin/dash-nvim "typescript:array.prototype.filter" "javascript:array.prototype.filter" "nodejs:array.prototype.filter"`

`bin/dash-nvim "array.prototype.filter"`

## Output

The CLI outputs a JSON array of objects. Each object has the following properties:

- `value` -- the number value of the item, to be used when selected. Running a query, then opening the URL `dash-workflow-callback://[value]` will open the selected item in Dash.app
- `ordinal` -- a value to sort by, currently this is the same value as `display`
- `display` -- a display value
- `keyword` -- the keyword (if there was one) on the query that returned this result
- `query` -- the full query that returned this result

**Note:** if running multiple queries, simply opening `dash-workflow-callback://[value]` may not work directly. Opening the URL assumes that
the value being opened was returned by the currently active query in Dash.app. You can work around this by just running the CLI again with
only the `query` value from the selected item, then opening `dash-workflow-callback://[value]`.
