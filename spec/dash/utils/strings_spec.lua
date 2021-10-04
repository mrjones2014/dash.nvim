local strings = require('dash.utils.strings')
local assert = require('luassert')

describe('urlencode', function()
  it('should replace each reserved character with its percent-encoded equivalent', function()
    local string_with_reserved_chars = "string-!#$%&'()*+,/:;=?@[]"
    local expectedResult = 'string-%21%23%24%25%26%27%28%29%2A%2B%2C%2F%3A%3B%3D%3F%40%5B%5D'
    assert.are.equal(strings.urlencode(string_with_reserved_chars), expectedResult)
  end)
end)

describe('join_list_to_string', function()
  it('should join a list of strings to a single string, each entry its own line', function()
    local list = { 'a', 'b', 'c' }
    local result = 'a\nb\nc'
    assert.are.equal(strings.join_list_to_string(list), result)
  end)
end)

describe('trim_trailing_newlines', function()
  it('should trim trailing newlines', function()
    local str = 'abc\n'
    local result = 'abc'
    assert.are.equal(strings.trim_trailing_newlines(str), result)
  end)
end)
