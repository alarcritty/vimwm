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

if let keyDown = CGEvent(keyboardEventSource: src, virtualKey: keyCode, keyDown: true),
   let keyUp = CGEvent(keyboardEventSource: src, virtualKey: keyCode, keyDown: false) {
    keyDown.flags = flags
    keyUp.flags = flags
    keyDown.post(tap: .cghidEventTap)
    keyUp.post(tap: .cghidEventTap)
}
