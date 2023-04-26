#![allow(dead_code)]

use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "author",
    version: 0:1:0,
    on_register: {
        register_function!("Initialize", initialize);
    }
}

fn initialize(player: Ref<IScriptable>) -> () {
    // here it seems return param can be typed
    let event = call!("CreateBootEvent;" () -> BootEvent);
    PlayerPuppet(player).queue_event(event.0);
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
struct BootEvent(Event);

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