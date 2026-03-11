import CoreGraphics
import Foundation

let args = CommandLine.arguments
let action = args.count >= 2 ? args[1] : "left"

guard let event = CGEvent(source: nil) else { exit(1) }
let pos = event.location

switch action {
case "left":
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: pos, mouseButton: .left)?
        .post(tap: .cghidEventTap)
    CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: pos, mouseButton: .left)?
        .post(tap: .cghidEventTap)
case "right":
    CGEvent(mouseEventSource: nil, mouseType: .rightMouseDown, mouseCursorPosition: pos, mouseButton: .right)?
        .post(tap: .cghidEventTap)
    CGEvent(mouseEventSource: nil, mouseType: .rightMouseUp, mouseCursorPosition: pos, mouseButton: .right)?
        .post(tap: .cghidEventTap)
case "double":
    guard let click1 = CGEvent(mouseEventSource: nil, mouseType: .leftMouseDown, mouseCursorPosition: pos, mouseButton: .left),
          let click2 = CGEvent(mouseEventSource: nil, mouseType: .leftMouseUp, mouseCursorPosition: pos, mouseButton: .left)
    else { exit(1) }
    click1.setIntegerValueField(.mouseEventClickState, value: 2)
    click1.post(tap: .cghidEventTap)
    click2.setIntegerValueField(.mouseEventClickState, value: 2)
    click2.post(tap: .cghidEventTap)
default:
    fputs("Usage: click [left|right|double]\n", stderr)
    exit(1)
}
