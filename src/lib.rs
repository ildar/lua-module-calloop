use mlua::prelude::*;
use calloop as rs_calloop;

mod timer;

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
        res.set("handle", lua.create_function(EventLoop::handle)?)?;
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
        let res = LoopSignal::new(lua,
                luaself.get::<_, LuaAnyUserData>("_self")?
                .borrow::<EventLoop>()?
                .0
                .get_signal()
            )?;
        Ok(res)
    }

    fn handle<'a>(lua: &'static Lua, luaself: LuaTable<'a>) -> LuaResult<LuaTable<'a>> {
        let res = LoopHandle::new(lua,
                luaself.get::<_, LuaAnyUserData>("_self")?
                .borrow::<EventLoop>()?
                .0
                .handle()
            )?;
        Ok(res)
    }

    fn run(_: &Lua, (luaself, timeout, mut data, cb): (LuaTable, u64, LuaTable<'static>, Option<LuaFunction>))
            -> LuaResult<()> {
        let ud: LuaAnyUserData = luaself.get("_self")?;
        let mut ref_self = ud.borrow_mut::<EventLoop<'static>>()?;
        ref_self.0.run(std::time::Duration::from_millis(timeout), &mut data,
            |data| {
                if cb.is_some() {
                    let cb = cb.as_ref();
                    cb.unwrap().call::<LuaTable,()>( (*data).clone() )
                        .unwrap();
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
    fn new(lua: &Lua, ls: rs_calloop::LoopSignal) -> LuaResult<LuaTable> {
        let res = lua.create_table()?;
        res.set("_self",
            lua.create_userdata(
                LoopSignal(ls)
            )?)?;
        res.set("stop", lua.create_function(LoopSignal::stop)?)?;
        res.set("wakeup", lua.create_function(LoopSignal::wakeup)?)?;
        Ok(res)
    }

    pub fn stop(_: &Lua, luaself: LuaTable) -> LuaResult<()> {
        let ud: LuaAnyUserData = luaself.get("_self")?;
        let ref_self = ud.borrow::<Self>()?;
        ref_self.0.stop();
        Ok(())
    }
    pub fn wakeup(_: &Lua, luaself: LuaTable) -> LuaResult<()> {
        let ud: LuaAnyUserData = luaself.get("_self")?;
        let ref_self = ud.borrow::<Self>()?;
        ref_self.0.wakeup();
        Ok(())
    }
}

// LoopHandle
#[derive(Clone)]
struct LoopHandle<'lh> (rs_calloop::LoopHandle<'lh, LuaTable<'lh>>);
impl LuaUserData for LoopHandle<'_> {}
impl LoopHandle<'_> {
    fn new<'lh>(lua: &'static Lua, lh: rs_calloop::LoopHandle<'static, LuaTable<'static>>) -> LuaResult<LuaTable<'lh>> {
        let res = lua.create_table()?;
        res.set("_self",
            lua.create_userdata(
                LoopHandle(lh)
            )?)?;
        res.set("insert_idle", lua.create_function(LoopHandle::insert_idle)?)?;
        Ok(res)
    }

    pub fn insert_idle(_: &Lua, (luaself, callback): (LuaTable, Option<LuaFunction<'static>>)) -> LuaResult<()> {
        let ud: LuaAnyUserData = luaself.get("_self")?;
        let ref_self = ud.borrow::<LoopHandle<'static>>()?;
        ref_self.0.insert_idle(
            move |data| {
                if callback.is_some() {
                    let cb = callback.as_ref();
                    cb.unwrap().call::<LuaTable,()>( (*data).clone() )
                        .unwrap();
                }
            });
        Ok(())
    }
}

