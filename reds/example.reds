
native func SumInts(ints: array<Int32>) -> Int32;
native func PluginName() -> String;
native func CreateTweakDBID(name: String) -> TweakDBID;
native func AppendToTweakDBID(base: TweakDBID, suffix: String) -> TweakDBID;
native func Impersonate(puppet: ref<ScriptedPuppet>) -> Void;

@addMethod(PlayerPuppet)
private func Greet() -> Void {
 GameObject.PlaySound(this, n"ono_v_greet");
}