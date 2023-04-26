#![allow(dead_code)]

use std::sync::Mutex;

use once_cell::sync::Lazy;
use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "author",
    version: 0:1:0,
    on_register: {
        register_function!("Initialize", initialize);
    }
}

static BIOMON: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::Idle));

fn initialize(controller: Ref<IScriptable>) -> () {
    let biomonitor = Biomonitor::Initialized {
        owner: BiomonitorControllerRS(controller),
        chemicals: Default::default(),
    };
    let event = Event(call!("NewObject;String" ("BootEvent".to_string()) -> Ref<IScriptable>));
    biomonitor.owner().unwrap().queue_event(event.0);
    *BIOMON.lock().unwrap() = State::Booting;
}

#[derive(Clone, Default)]
#[repr(transparent)]
struct BiomonitorControllerRS(Ref<IScriptable>);

#[derive(Clone, Default)]
#[repr(transparent)]
struct Event(Ref<IScriptable>);

#[derive(Clone, Default)]
enum Biomonitor {
    #[default]
    Uninitialized,
    Initialized {
        owner: BiomonitorControllerRS,
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
    fn owner(&self) -> Option<&BiomonitorControllerRS> {
        match self {
            Biomonitor::Uninitialized => None,
            Biomonitor::Initialized { owner, .. } => Some(owner),
        }
    }
}

#[redscript_import]
impl BiomonitorControllerRS {
    #[redscript(native)]
    fn queue_event(&self, event: Ref<IScriptable>) -> ();
}

unsafe impl NativeRepr for BiomonitorControllerRS {
    const NAME: &'static str = "handle:BiomonitorControllerRS";
}

#[derive(Clone, Debug, Default)]
#[repr(i64)]
enum State {
    #[default]
    Idle = 0,
    Booting = 1,
    Analyzing = 2,
    Summarizing = 3,
    Closing = 4,
    Dismissing = 5,
}
