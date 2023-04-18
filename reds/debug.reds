module TestIt

import TestIt.AlsoRefs

// give it a try in CET console with:
// Game.GetPlayer():TrySumInts()
@addMethod(PlayerPuppet)
public func TrySumInts() -> Void {
 let result = SumInts([2000, 77]);
 LogChannel(n"DEBUG", s"[RED] SumInts: \(result)");
}

// Game.GetPlayer():TryRefsExample()
@addMethod(PlayerPuppet)
public func TryRefsExample() -> Void {
 AlsoRefs(this);
}