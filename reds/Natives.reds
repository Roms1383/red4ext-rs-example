native func Initialize() -> Void;

public class Controller extends inkIGameController {
 static func Create() -> ref<Controller> { 
  LogChannel(n"DEBUG", "create Controller");
  return new Controller(); }
 protected cb func OnBoot(evt: ref<BootEvent>) -> Bool {
  LogChannel(n"DEBUG", s"on boot");
 }
}

class BootEvent extends Event {}

class Events {
 static func Boot() -> ref<Event> {
  return new BootEvent();
 }
}