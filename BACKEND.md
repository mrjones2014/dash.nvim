# Rust Backend

The Rust backend is a command-line interface. A pre-built binary can be found at `bin/dash-nvim`.

To build from source, you will need a Rust toolchain, which can be installed from [rustup.rs](https://rustup.rs).
Once this is installed, you should be able to build via `make build-rust`.

## Arguments

| Argument Flag                                 | Argument Name  | Description                                                                                  | Required |
| --------------------------------------------- | -------------- | -------------------------------------------------------------------------------------------- | :------: |
| `-c`                                          | `cli_path`     | Path to Dash.app if using a non-standard path, defaults to `/Applications/Dash.app`          |          |
| `--pretty-print`                              | `pretty_print` | Pretty-print the JSON output, useful for debugging purposes, defaults to `false`             |          |
| (positional, must come after other arguments) | `queries`      | A space-separated list of queries to run, queries can contain spaces if surrounded in quotes |    ✅    |

### Examples

`bin/dash-nvim -c /path/to/Dash.app "typescript:array.prototype.filter" "javascript:array.prototype.filter" "nodejs:array.prototype.filter"`

`bin/dash-nvim "typescript:array.prototype.filter" "javascript:array.prototype.filter" "nodejs:array.prototype.filter"`

`bin/dash-nvim "array.prototype.filter"`

`bin/dash-nvim --pretty-print "array.prototype.filter"`

`bin/dash-nvim -c /path/to/Dash.app --pretty-print "array.prototype.filter"`

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
