use mlua::prelude::*;
use calloop as rs_calloop;

#[mlua::lua_module]
fn calloop(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let el = lua.create_table()?;
    el.set("new", lua.create_function(EventLoop_new)?)?;
    exports.set("EventLoop", el)?;
    Ok(exports)
}

// EventLoop
#[allow(non_snake_case)]
fn EventLoop_new(lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    let res = lua.create_table()?;
    Ok(res)
}
