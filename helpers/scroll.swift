import CoreGraphics
import Foundation

let args = CommandLine.arguments
let dy = args.count >= 2 ? Int(args[1]) ?? 0 : 0

guard let scrollEvent = CGEvent(scrollWheelEvent2Source: nil, units: .line, wheelCount: 1, wheel1: Int32(dy), wheel2: 0, wheel3: 0) else { exit(1) }
scrollEvent.post(tap: .cghidEventTap)
