local dash = require('dash')
return require('telescope').register_extension({
  setup = dash.setup,
  exports = {
    search = dash.search,
  },
})
