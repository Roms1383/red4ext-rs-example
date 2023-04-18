use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "jekky",
    version: 1:0:0,
    on_register: {
        register_function!("SumInts", sum_ints);
        register_function!("TestIt.AlsoRefs", also_refs);
    }
}

fn sum_ints(ints: Vec<i32>) -> i32 {
    log(
        CName::new("DEBUG"),
        format!(
            "received: {}",
            ints.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ),
    );
    ints.iter().sum()
}

/// show how to call .reds static native function back with multiple parameters
fn log(channel: CName, message: impl Into<String>) {
    let message = format!("[RUST] {}", message.into());
    // call!("LogChannel" (channel, message) -> ()); crashes
    // call!("LogChannel;" (channel, message) -> ()); crashes
    // call!("LogChannel;CNameString" (channel, message) -> ()); crashes
    call!("TestIt.LogIt;CNameString" (channel, message) -> ());
}

/// ref<IScriptable> can also be sent back and forth, and called methods onto
fn also_refs(instance: Ref<ffi::IScriptable>) {
    let is_player = call!(instance, "IsExactlyA" (CName::new("PlayerPuppet")) -> bool); // method from IScriptable
    let is_replacer = is_player && call!(instance, "IsReplacer" () -> bool); // methode from PlayerPuppet
    match (is_player, is_replacer) {
        (false, _) => log(
            CName::new("DEBUG"),
            "call method from instance: is NOT a player",
        ),
        (true, true) => log(
            CName::new("DEBUG"),
            "call method from instance: is a player AND is a replacer",
        ),
        (true, false) => log(
            CName::new("DEBUG"),
            "call method from instance: is a player BUT is NOT a replacer",
        ),
    };
}
