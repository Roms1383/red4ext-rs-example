import Codeware.UI.inkCustomController

/// invoke in CET like:
/// ```lua
/// Initialize(NewObject("BiomonitorControllerRS"));
/// ```
native func Initialize(controller: ref<IScriptable>) -> Void;

public class BiomonitorControllerRS extends inkCustomController {
 protected cb func OnBoot(evt: ref<BootEvent>) -> Void {
  LogChannel(n"DEBUG", s"on boot");
 }
}

public static func CreateBootEvent() -> ref<Event> { return new BootEvent() as Event; }

class BootEvent extends Event {}