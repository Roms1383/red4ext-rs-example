#![allow(dead_code)]

use std::cell::RefCell;

use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "author",
    version: 0:1:0,
    on_register: {
        register_function!("Initialize", initialize);
    }
}

thread_local!(static TIMES: RefCell<u32> = RefCell::new(0));
thread_local!(static PLAYER: RefCell<Option<PlayerPuppet>> = RefCell::new(None));

#[redscript_global(name = "BootEvent::New")]
fn create_boot_event() -> BootEvent;

fn initialize(player: Ref<IScriptable>) -> () {
    // here it return param can be typed
    let event = create_boot_event();
    TIMES.with(|f| {
        *f.borrow_mut() += 1;
    });
    // SAFETY: ok this is just an experiment
    // handle could change over time
    // also Ref<IScriptable> currently does not impl PartialEq
    PLAYER.with(|f| {
        let mut v = f.borrow_mut();
        if v.is_none() {
            *v = Some(PlayerPuppet(player.clone()));
        }
    });
    event.set_times(TIMES.with(|f| *f.borrow()));
    PLAYER.with(|f| f.borrow().as_ref().unwrap().queue_event(Event(event.0)));
    info!("@initialize");
}

#[derive(Clone, Default)]
#[repr(transparent)]
struct Event(Ref<IScriptable>);

unsafe impl NativeRepr for Event {
    const NAME: &'static str = "handle:redEvent";
}

#[derive(Default, Clone)]
#[repr(transparent)]
struct BootEvent(Ref<IScriptable>);

#[redscript_import]
impl Event {}

#[redscript_import]
impl BootEvent {
    #[redscript]
    fn set_times(&self, times: u32) -> ();
}

unsafe impl NativeRepr for BootEvent {
    const NAME: &'static str = "handle:BootEvent";
}

#[derive(Clone, Default)]
#[repr(transparent)]
struct PlayerPuppet(Ref<IScriptable>);

#[redscript_import]
impl PlayerPuppet {
    // here this HAS to be Event (handle:redEvent)
    #[redscript(native)]
    fn queue_event(&self, event: Event) -> ();
}

unsafe impl NativeRepr for PlayerPuppet {
    const NAME: &'static str = "handle:PlayerPuppet";
}
