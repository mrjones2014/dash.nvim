local M = {}

local function parseResults(xmlString)
  local xml = require('xml2lua')
  local handler = require('xmlhandler.tree')
  local parser = xml.parser(handler)
  parser:parse(xmlString)
  print(vim.inspect(handler.root))
end

function M.search(query)
  local utils = require('dash.utils')
  local result = utils.runSearch(query)
  local stdout = result.stdout
  local stderr = result.stderr

  if stdout ~= nil then
    parseResults(stdout)
  end

  if stderr ~= nil then
    print(stderr)
  end
end

return M
