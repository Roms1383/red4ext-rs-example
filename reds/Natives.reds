import Codeware.UI.inkCustomController

/// invoke in CET like:
/// ```lua
/// Initialize(Game.GetPlayer());
/// ```
native func Initialize(player: ref<IScriptable>) -> Void;

public static func CreateBootEvent() -> ref<BootEvent> { return new BootEvent(); }

class BootEvent extends Event {}

@addMethod(PlayerPuppet)
protected cb func OnBoot(evt: ref<BootEvent>) -> Void {
 LogChannel(n"DEBUG", s"on boot");
}

@addMethod(PlayerPuppet)
protected cb func OnAnyEvent(evt: ref<Event>) -> Void {
 LogChannel(n"DEBUG", s"on any event: \(NameToString(evt.GetClassName()))");
}
