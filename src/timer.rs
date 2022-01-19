use mlua::prelude::*;
//use calloop as rs_calloop;
use calloop::timer as rs_timer;

#[mlua::lua_module]
fn calloop_timer(lua: &'static Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let tm = lua.create_table()?;
    tm.set("new", lua.create_function(Timer::new)?)?;
    exports.set("Timer", tm)?;
    Ok(exports)
}

// Timer
struct Timer<'tm> (rs_timer::Timer<LuaValue<'tm>>);
impl LuaUserData for Timer<'_> {}
impl<'tm> Timer<'tm> {
    fn new(lua: &'static Lua, _: ()) -> LuaResult<LuaTable<'tm>> {
        let res = lua.create_table()?;
        res.set("_EventSource", "timer::Timer")?;
        res.set("_self",
            lua.create_userdata(
                Timer(rs_timer::Timer::new()?)
            )?)?;
        res.set("handle", lua.create_function(Timer::handle)?)?;
        Ok(res)
    }

    pub fn handle(lua: &'static Lua, luaself: LuaTable) -> LuaResult<LuaTable<'tm>> {
        let ud: LuaAnyUserData = luaself.get("_self")?;
        let ref_self = ud.borrow::<Timer<'static>>()?;
        Ok( TimerHandle::new(lua, ref_self.0.handle())? )
    }
}

// TimerHandle
#[derive(Clone)]
struct TimerHandle<'th> (rs_timer::TimerHandle<LuaValue<'th>>);
impl LuaUserData for TimerHandle<'_> {}
impl<'th> TimerHandle<'th> {
    fn new(lua: &'static Lua, th: rs_timer::TimerHandle<LuaValue<'static>>) -> LuaResult<LuaTable<'th>> {
        let res = lua.create_table()?;
        res.set("_self",
            lua.create_userdata(
                TimerHandle(th)
            )?)?;
        res.set("add_timeout", lua.create_function(TimerHandle::add_timeout)?)?;
        Ok(res)
    }

    // TODO: should return rs_timer::Timeout
    pub fn add_timeout(_: &Lua, (luaself, delay_from_now, data): (LuaTable, u64, LuaValue<'static>)) -> LuaResult<()> {
        let ud: LuaAnyUserData = luaself.get("_self")?;
        let ref_self = ud.borrow::<TimerHandle<'static>>()?;
        ref_self.0.add_timeout(std::time::Duration::from_millis(delay_from_now), data);
        Ok(())
    }
}

