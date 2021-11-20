describe("require('libdash_nvim')", function()
  it("should be able to require('libdash_nvim')", function()
    local ok, libdash = pcall(require, 'libdash_nvim')

    -- assert we can require the backend module
    assert.is_true(ok)
    assert.is_true(libdash ~= nil)

    -- assert libdash.query function exists
    assert.is_true(libdash.query ~= nil)
    assert.are.equal('function', type(libdash.query))

    -- assert libdash.open_item function exists
    assert.is_true(libdash.open_item ~= nil)
    assert.are.equal('function', type(libdash.open_item))

    -- assert libdash.open_url function exists
    assert.is_true(libdash.open_url ~= nil)
    assert.are.equal('function', type(libdash.open_url))

    -- assert libdash.setup function exist
    assert.is_true(libdash.setup ~= nil)
    assert.are.equal('function', type(libdash.setup))

    -- assert libdash.config table exists
    assert.is_true(libdash.config ~= nil)
    assert.are.equal('table', type(libdash.config))

    -- assert constants exist
    assert.is_true(#libdash.DASH_APP_BASE_PATH > 0)
    assert.is_true(#libdash.DASH_APP_CLI_PATH > 0)
  end)

  describe('.setup', function()
    it('should merge config with default config', function()
      local libdash = require('libdash_nvim')

      libdash.setup({
        search_engine = 'google',
        file_type_keywords = { lua = 'test' },
      })

      assert.are.equal(libdash.config.dash_app_path, libdash.default_config.dash_app_path)
      assert.are.equal('google', libdash.config.search_engine)
      assert.are.equal('test', libdash.config.file_type_keywords.lua)
      assert.is_true(libdash.default_config.file_type_keywords.rust)
    end)
  end)
end)
