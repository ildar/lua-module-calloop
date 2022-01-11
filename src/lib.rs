use mlua::prelude::*;
use calloop as rs_calloop;

#[mlua::lua_module]
fn calloop(lua: &'static Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let el = lua.create_table()?;
    el.set("new", lua.create_function(EventLoop::new)?)?;
    exports.set("EventLoop", el)?;
    Ok(exports)
}

// EventLoop
struct EventLoop<'a> (rs_calloop::EventLoop<'a, LuaTable<'a>>);
impl LuaUserData for EventLoop<'_> {}
impl EventLoop<'_> {
    fn new(lua: &'static Lua, _: ()) -> LuaResult<LuaTable> {
        let res = lua.create_table()?;
        let el = EventLoop(rs_calloop::EventLoop::try_new()?);
        res.set("_self", lua.create_userdata(el)?)?;
        res.set("get_signal", lua.create_function(EventLoop::get_signal)?)?;
        res.set("run", lua.create_function(EventLoop::run)?)?;
        Ok(res)
    }

    /* TODO:
    fn import<'a>(lua: &'a Lua, el: rs_calloop::EventLoop<rs_calloop::LoopSignal>) -> LuaResult<LuaTable<'a>> {
        let res = lua.create_table()?;
        let el = EventLoop(el);
        res.set("_self", lua.create_userdata(el)?)?;
        Ok(res)
    }
    */

    fn get_signal<'a>(lua: &'a Lua, luaself: LuaTable<'a>) -> LuaResult<LuaTable<'a>> {
        let res: LuaTable = luaself.get("_signal").unwrap_or({
            let res = lua.create_table()?;
            res.set("_self",
                lua.create_userdata(
                    LoopSignal(
                        luaself.get::<_, LuaAnyUserData>("_self")?
                        .borrow_mut::<EventLoop>()?
                        .0
                        .get_signal()
                    )
                )?)?;
            luaself.set("_signal", res.clone())?;
            res
            });
        Ok(res)
    }

    fn run(_: &Lua, (luaself, timeout, mut data, cb): (LuaTable, u64, LuaTable<'static>, Option<LuaFunction>))
            -> LuaResult<()> {
        let ud: LuaAnyUserData = luaself.get("_self")?;
        let mut ref_self = ud.borrow_mut::<EventLoop<'static>>()?;
        ref_self.0.run(std::time::Duration::from_millis(timeout), &mut data,
            |_|{
                if cb.is_some() {
                    let cb=cb.as_ref();
                    cb.unwrap().call::<(),()>(());
                }
            })?;
        Ok(())
    }
}

// LoopSignal
#[derive(Clone)]
struct LoopSignal (rs_calloop::LoopSignal);
impl LuaUserData for LoopSignal {}
impl LoopSignal {
}

