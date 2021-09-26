local tableUtils = require('dash.utils.tables')
local assert = require('luassert')

describe('concatArrays', function()
  it('should concatenate array tables together, leaving both original arrays unmodified', function()
    local arr1 = { '1', '2', '3' }
    local arr2 = { '4', '5', '6' }
    local result = tableUtils.concatArrays(arr1, arr2)

    assert.are.equal(3, #arr1)
    assert.are.equal(3, #arr2)
    assert.are.equal(6, #result)

    assert.are.equal(arr1[1], '1')
    assert.are.equal(arr1[2], '2')
    assert.are.equal(arr1[3], '3')

    assert.are.equal(arr2[1], '4')
    assert.are.equal(arr2[2], '5')
    assert.are.equal(arr2[3], '6')

    assert.are.equal(result[1], '1')
    assert.are.equal(result[2], '2')
    assert.are.equal(result[3], '3')
    assert.are.equal(result[4], '4')
    assert.are.equal(result[5], '5')
    assert.are.equal(result[6], '6')
  end)
end)

describe('cloneTableByValue', function()
  it('should return a new table with a new reference but all the same values', function()
    local source = {
      a = 1,
      b = {
        c = 2,
      },
    }
    local result = tableUtils.cloneTableByValue(source)
    assert.are_not.equal(source, result)
    assert.are.equal(source.a, result.a)
    assert.are.equal(source.b.c, result.b.c)
  end)
end)

describe('mergeTables', function()
  it('should merge tables recursively, leaving each source table unmodified', function()
    local tbl1 = {
      a = 1,
      b = {
        c = 2,
      },
    }
    local tbl2 = {
      b = {
        d = 3,
      },
    }
    local result = tableUtils.mergeTables(tbl1, tbl2)

    assert.is_nil(tbl1.b.d)
    assert.is_nil(tbl2.a)
    assert.is_nil(tbl2.b.c)

    assert.are.equal(2, result.b.c)
    assert.are.equal(3, result.b.d)
  end)

  it('should overwrite values from the first table with values from the second table', function()
    local tbl1 = {
      a = 1,
      b = 2,
      c = {
        d = 3,
      },
    }
    local tbl2 = {
      b = 3,
      c = {
        d = 4,
      },
    }
    local result = tableUtils.mergeTables(tbl1, tbl2)

    assert.are.equal(3, result.b)
    assert.are.equal(4, result.c.d)
  end)
end)
