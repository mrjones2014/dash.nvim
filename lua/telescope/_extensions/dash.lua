return require('telescope').register_extension({
  setup = function(config)
    require('dash').setup(config)
  end,
  health = function()
    require('dash.health').check()
  end,
  exports = {
    search = function(bang)
      require('dash').search(bang)
    end,
    search_no_filter = function()
      require('dash').search(true)
    end,
  },
})
