module BiomonitorExt

public class BiomonitorController extends inkIGameController {
 static func Create() -> ref<IScriptable> { 
  LogChannel(n"DEBUG", "create BiomonitorController");
  return new BiomonitorController() as IScriptable; }
 protected cb func OnBoot(evt: ref<BootEvent>) -> Bool {
  LogChannel(n"DEBUG", s"on boot");
 }
}

class BootEvent extends Event {}

class BiomonitorEvents {
 static func Boot() -> ref<Event> {
  return new BootEvent();
 }
}