import CoreGraphics
import Foundation

let args = CommandLine.arguments
guard args.count >= 2, let keyCode = UInt16(args[1]) else {
    fputs("Usage: key <keycode> [shift] [cmd] [alt] [ctrl]\n", stderr)
    exit(1)
}

var flags: CGEventFlags = []
for i in 2..<args.count {
    switch args[i] {
    case "shift": flags.insert(.maskShift)
    case "cmd":   flags.insert(.maskCommand)
    case "alt":   flags.insert(.maskAlternate)
    case "ctrl":  flags.insert(.maskControl)
    default: break
    }
}

let src = CGEventSource(stateID: .hidSystemState)

let heldFlags = CGEventSource.flagsState(.combinedSessionState)
let modKeys: [(CGEventFlags, CGKeyCode)] = [
    (.maskShift, 56),
    (.maskCommand, 55),
    (.maskAlternate, 58),
    (.maskControl, 59),
]
let toRelease = modKeys.filter { (mask, _) in
    heldFlags.contains(mask) && !flags.contains(mask)
}

for (_, code) in toRelease {
    if let e = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: false) {
        e.post(tap: .cghidEventTap)
    }
}

if let keyDown = CGEvent(keyboardEventSource: src, virtualKey: keyCode, keyDown: true),
   let keyUp = CGEvent(keyboardEventSource: src, virtualKey: keyCode, keyDown: false) {
    keyDown.flags = flags
    keyUp.flags = flags
    keyDown.post(tap: .cghidEventTap)
    keyUp.post(tap: .cghidEventTap)
}

for (_, code) in toRelease {
    if let e = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: true) {
        e.post(tap: .cghidEventTap)
    }
}
