
native func SumInts(ints: array<Int32>) -> Int32;
native func PluginName() -> String;
native func CreateTweakDBID(name: String) -> TweakDBID;
native func AppendToTweakDBID(base: TweakDBID, suffix: String) -> TweakDBID;
native func Consume(consumptions: ref<Consumptions>, consumable: Consumable) -> Void;
native func Attach(watcher: ref<IScriptable>) -> ref<IScriptable>;

class MyEntityWatcher extends ScriptableSystem {
 private let m_callbackSystem: wref<CallbackSystem>;
 private func OnAttach() {
  this.m_callbackSystem = Attach(this as IScriptable) as CallbackSystem;
 }
 private cb func OnEntityAssemble(event: ref<EntityLifecycleEvent>) {
   let entity = event.GetEntity();

   if entity.GetTemplatePath() == r"base\\characters\\entities\\player\\player_ma_fpp.ent" {
     LogChannel(n"DEBUG", s"male V assembled!");

     this.m_callbackSystem.UnregisterCallback(n"Entity/Assemble", this);
   }
 }
}

public class Consumptions {
 func RemoteConsume(consumable: Consumable) -> Void {
  LogChannel(n"DEBUG", s"called with consumable \(ToString(consumable))");
 }
 /// pipe to global native
 public func Consume(consumable: Consumable) -> Void {
  Consume(this, consumable);
 }
 static func New() -> ref<Consumptions> { return new Consumptions(); }
}
enum Consumable {
 MaxDOC = 0,
 BounceBack = 1,
}