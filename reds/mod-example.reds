module TestIt

native func AlsoRefs(instance: ref<IScriptable>) -> Void;

public static func LogIt(channel: CName, message: String) -> Void {
 LogChannel(channel, message);
}