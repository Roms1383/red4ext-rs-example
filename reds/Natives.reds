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

class BootEvent extends Event {}