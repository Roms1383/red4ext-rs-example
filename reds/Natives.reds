import Codeware.UI.inkCustomController

/// invoke in CET like:
/// ```lua
/// Initialize(Game.GetPlayer());
/// ```
native func Initialize(player: ref<IScriptable>) -> Void;

public static func CreateBootEvent() -> ref<BootEvent> { return new BootEvent(); }

class BootEvent extends Event {
 public let times: Uint32;
 func SetTimes(times: Uint32) -> Void { this.times = times; }
}

@addMethod(PlayerPuppet)
protected cb func OnBoot(evt: ref<BootEvent>) -> Void {
 LogChannel(n"DEBUG", s"on boot \(evt.times) time(s)");
}

@addMethod(PlayerPuppet)
protected cb func OnAnyEvent(evt: ref<Event>) -> Void {
 LogChannel(n"DEBUG", s"on any event: \(NameToString(evt.GetClassName()))");
}
