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

    -- assert libdash.build_query function exists
    assert.is_true(libdash.build_query ~= nil)
    assert.are.equal('function', type(libdash.build_query))

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

  describe('.build_query', function()
    it('when bang=true and file_type_keywords value is boolean true, then should not prefix with keywords', function()
      local search_text = 'match arms'
      local buffer_type = 'rust'
      local bang = true
      local file_type_keywords = { rust = true }

      local results = require('libdash_nvim').build_query(search_text, buffer_type, bang, file_type_keywords)

      assert.are.equal(1, #results)
      assert.are.equal(search_text, results[1])
    end)

    it('when bang=true and file_type_keywords value is a table, then should not prefix with keywords', function()
      local search_text = 'match arms'
      local buffer_type = 'rust'
      local bang = true
      local file_type_keywords = { rust = { 'rust' } }

      local results = require('libdash_nvim').build_query(search_text, buffer_type, bang, file_type_keywords)

      assert.are.equal(1, #results)
      assert.are.equal(search_text, results[1])
    end)

    it(
      'when bang=false and file_type_keywords value is boolean true, then should return single query prefixed '
        .. 'with buffer type',
      function()
        local search_text = 'match arms'
        local buffer_type = 'rust'
        local bang = false
        local file_type_keywords = { rust = true }

        local results = require('libdash_nvim').build_query(search_text, buffer_type, bang, file_type_keywords)

        assert.are.equal(1, #results)
        assert.are.equal(buffer_type .. ':' .. search_text, results[1])
      end
    )

    it(
      'when bang=false and file_type_keywords value is table, then should return table of queries prefixed '
        .. 'with each value from the file_type_keywords table',
      function()
        local search_text = 'array'
        local buffer_type = 'javascript'
        local bang = false
        local file_type_keywords = { javascript = { 'javascript', 'nodejs' } }

        local results = require('libdash_nvim').build_query(search_text, buffer_type, bang, file_type_keywords)

        assert.are.equal(2, #results)
        assert.are.equal(buffer_type .. ':' .. search_text, results[1])
        assert.are.equal('nodejs:' .. search_text, results[2])
      end
    )

    it(
      'when bang=false and file_type_keywords value is boolean false, should return single query without prefix',
      function()
        local search_text = 'match arms'
        local buffer_type = 'rust'
        local bang = false
        local file_type_keywords = { rust = false }

        local results = require('libdash_nvim').build_query(search_text, buffer_type, bang, file_type_keywords)

        assert.are.equal(1, #results)
        assert.are.equal(search_text, results[1])
      end
    )

    it(
      'when bang=false and file_type_keywords value is an invalid value, then assume no file type filtering',
      function()
        local search_text = 'match arms'
        local buffer_type = 'rust'
        local bang = false
        local file_type_keywords = { rust = 123 }

        local results = require('libdash_nvim').build_query(search_text, buffer_type, bang, file_type_keywords)

        assert.are.equal(1, #results)
        assert.are.equal(search_text, results[1])
      end
    )
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
