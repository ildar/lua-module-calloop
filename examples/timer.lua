local calloop = require "calloop"
local timer = require "calloop.timer"

-- main
local event_loop = calloop.EventLoop.new()
assert(event_loop, "Failed to initialize the event loop!")

local handle = event_loop:handle()

local source = timer.Timer.new()
assert(source, "Failed to create timer event source!")

local timer_handle = source:handle()
timer_handle:add_timeout(5000, "Timeout reached!")
      
handle:insert_source(source, function (_shared_data)
    print("Event fired: {}")
    _shared_data:stop()
  end)
-- TODO: .expect("Failed to insert event source!");

shared_data = event_loop:get_signal()

event_loop:run(20, shared_data)
-- TODO: .expect("Error during event loop!");
