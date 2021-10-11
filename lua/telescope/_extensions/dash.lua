return require('telescope').register_extension({
  setup = function(config)
    require('dash').setup(config)
  end,
  health = function()
    require('dash.health').check()
  end,
  exports = {
    search = function(...)
      require('dash').search(...)
    end,
    search_no_filter = function(...)
      require('dash').search(true, ...)
    end,
  },
})
