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

static mut BIOMON: Biomonitor = Biomonitor::Uninitialized;

fn initialize() {
    let controller = Controller(Controller::create());
    let biomonitor = Biomonitor::Initialized {
        owner: controller,
        chemicals: Default::default(),
    };
    let event = Event(call!("Events.Boot;" () -> Ref<IScriptable>));
    biomonitor.owner().unwrap().queue_event(event.0);
    unsafe {
        BIOMON = biomonitor.clone();
    }
}

#[derive(Clone, Default)]
#[repr(transparent)]
struct Controller(Ref<IScriptable>);

#[derive(Clone, Default)]
#[repr(transparent)]
struct Event(Ref<IScriptable>);

#[derive(Clone, Default)]
enum Biomonitor {
    #[default]
    Uninitialized,
    Initialized {
        owner: Controller,
        chemicals: Vec<Ref<IScriptable>>,
    },
}

impl Biomonitor {
    fn chemicals(&self) -> Option<&[Ref<IScriptable>]> {
        match self {
            Biomonitor::Uninitialized => None,
            Biomonitor::Initialized { chemicals, .. } => Some(&chemicals),
        }
    }
    fn owner(&self) -> Option<&Controller> {
        match self {
            Biomonitor::Uninitialized => None,
            Biomonitor::Initialized { owner, .. } => Some(owner),
        }
    }
}

#[redscript_import]
impl Controller {
    fn create() -> Ref<IScriptable>;

    #[redscript(native)]
    fn queue_event(&self, event: Ref<IScriptable>) -> ();
}

// #[derive(Clone, Debug, Default)]
// #[repr(i64)]
// enum State {
//     #[default]
//     Idle = 0,
//     Booting = 1,
//     Analyzing = 2,
//     Summarizing = 3,
//     Closing = 4,
//     Dismissing = 5,
// }

// unsafe impl NativeRepr for State {
//     const NAME: &'static str = "State";
// }
