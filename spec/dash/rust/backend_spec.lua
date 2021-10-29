describe("require('libdash_nvim')", function()
  it("should be able to require('libdash_nvim')", function()
    local ok, libdash = pcall(require, 'libdash_nvim')
    assert.is_true(ok)
    assert.is_true(libdash ~= nil)
    assert.is_true(libdash.query ~= nil)
    assert.are.equal('function', type(libdash.query))
    assert.is_true(libdash.open_item ~= nil)
    assert.are.equal('function', type(libdash.open_item))
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
end)
