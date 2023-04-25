use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "jekky",
    version: 1:0:0,
    on_register: {
        register_function!("SumInts", sum_ints);
        register_function!("PluginName", plugin_name);
        register_function!("CreateTweakDBID", create_tweakdb_id);
        register_function!("AppendToTweakDBID", append_to_tweakdb_id);
        register_function!("Consume", consume);
        register_function!("Attach", attach);
    }
}

/// SumInts
///
/// test in CET like:
/// ```lua
/// LogChannel(CName.new("DEBUG"), SumInts({2000, 77}));
/// ```
fn sum_ints(ints: Vec<i32>) -> i32 {
    ints.iter().sum()
}

/// PluginName
///
/// test in CET like:
/// ```lua
/// LogChannel(CName.new("DEBUG"), PluginName());
/// ```
fn plugin_name() -> String {
    String::from("RED4EXT.RS.EXAMPLE")
}

/// CreateTweakDBID
///
/// test in CET like:
/// ```lua
/// LogChannel(CName.new("DEBUG"), TDBID.ToStringDEBUG(CreateTweakDBID("A.Test")));
/// ```
fn create_tweakdb_id(name: String) -> TweakDBID {
    TweakDBID::new(&name)
}

/// AppendToTweakDBID
///
/// test in CET like:
/// ```lua
/// LogChannel(CName.new("DEBUG"), TDBID.Create("A.Test") == AppendToTweakDBID(TDBID.Create("A."), "Test"));
/// ```
fn append_to_tweakdb_id(base: TweakDBID, suffix: String) -> TweakDBID {
    TweakDBID::new_from_base(&base, suffix.as_str())
}

/// Consume
///
/// test in CET like:
/// ```lua
/// Consume(NewObject("Consumptions"), Consumable.BlackLace);
/// // or
/// NewObject("Consumptions").Consume(Consumable.BlackLace);
/// ```
fn consume(consumptions: Ref<IScriptable>, consumable: Consumable) {
    Consumptions(consumptions).consume(consumable);
}

#[derive(Clone, Default)]
#[repr(transparent)]
struct Consumptions(Ref<IScriptable>);

#[redscript_import]
impl Consumptions {
    pub fn remote_consume(&self, consumable: Consumable) -> ();
}

impl Consume for Consumptions {
    fn consume(&self, consumable: Consumable) {
        self.remote_consume(consumable);
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(i64)]
pub enum Consumable {
    #[default]
    MaxDOC = 0,
    BounceBack = 1,
}

unsafe impl NativeRepr for Consumable {
    const NAME: &'static str = "Consumable";
}

pub trait Consume {
    fn consume(&self, consumable: Consumable);
}

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct GameInstance;

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct CallbackSystem(Ref<IScriptable>);

#[redscript_import]
impl GameInstance {
    fn get_callback_system() -> Ref<IScriptable>;
}

#[redscript_import]
impl CallbackSystem {
    /// public native func RegisterCallback(event: CName, target: ref<IScriptable>, function: CName, opt sticky: Bool)
    #[redscript(native)]
    pub fn register_callback(&self, event: CName, target: Ref<IScriptable>, func: CName) -> ();
}

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct MyEntityWatcher(Ref<IScriptable>);

fn attach(watcher: Ref<IScriptable>) -> Ref<IScriptable> {
    let system = CallbackSystem(GameInstance::get_callback_system());
    system.register_callback(
        CName::new("Entity/Assemble"),
        watcher,
        CName::new("OnEntityAssemble"),
    );
    system.0
}
