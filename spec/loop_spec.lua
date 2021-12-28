local calloop = require "calloop"

describe("calloop Rust module's", function()
  describe("main loop", function()
    it("can be created", function()
      local el = calloop.EventLoop.new()
      assert.is_not_nil(el)
    end)
  end)
end)
