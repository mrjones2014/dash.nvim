local dash = require('dash')
return require('telescope').register_extension({
  setup = dash.setup,
  exports = {
    search = dash.search,
    search_no_filter = function()
      dash.search(true)
    end,
  },
})
