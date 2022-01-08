local calloop = require "calloop"

describe("calloop Rust module's", function()
  describe("main loop", function()
    setup(function()
      el = calloop.EventLoop.new()
    end)
    it("can be created", function()
      assert.is_not_nil(el)
    end)
    pending("can be run", function()
      assert.is_not_nil(el.run)
      el:run(500, {}, function ()
            -- TODO: emm, quit the loop somehow?
          end)
    end)
  end)
end)
