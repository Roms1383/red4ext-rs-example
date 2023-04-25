
native func SumInts(ints: array<Int32>) -> Int32;
native func PluginName() -> String;
native func CreateTweakDBID(name: String) -> TweakDBID;
native func AppendToTweakDBID(base: TweakDBID, suffix: String) -> TweakDBID;
native func Consume(consumptions: ref<Consumptions>, consumable: Consumable) -> Void;

public class Consumptions {
 public func ConsumeReds(consumable: Consumable) -> Void {
  LogChannel(n"DEBUG", s"called with consumable \(ToString(consumable)) \(consumable) \(EnumInt(consumable))");
 }
 static func New() -> ref<Consumptions> { return new Consumptions(); }
}
enum Consumable {
 MaxDOC = 0,
 BounceBack = 1,
}