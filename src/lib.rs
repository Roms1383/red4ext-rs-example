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

thread_local!(static BIOMON: RefCell<Biomonitor> = RefCell::new(Biomonitor::Uninitialized));

fn initialize(controller: Ref<IScriptable>) -> () {
    let biomonitor = Biomonitor::Initialized {
        owner: BiomonitorControllerRS(controller),
        chemicals: Default::default(),
    };
    // here it seems return param can be typed
    let event = call!("CreateBootEvent;" () -> Event);
    biomonitor.owner().unwrap().queue_event(event);
    BIOMON.with(move |biomon| {
        *biomon.borrow_mut() = biomonitor;
    });
    info!("biomonitor initialized");
}

#[derive(Clone, Default)]
#[repr(transparent)]
struct BiomonitorControllerRS(Ref<IScriptable>);

#[derive(Clone, Default)]
#[repr(transparent)]
struct Event(Ref<IScriptable>);

unsafe impl NativeRepr for Event {
    const NAME: &'static str = "handle:redEvent";
}

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
    // here this HAS to be Event (handle:redEvent)
    #[redscript(native)]
    fn queue_event(&self, event: Event) -> ();
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

#[derive(Default, Clone)]
#[repr(transparent)]
struct BootEvent(Ref<IScriptable>);

unsafe impl NativeRepr for BootEvent {
    const NAME: &'static str = "handle:BootEvent";
}