import CoreGraphics
import Foundation

let args = CommandLine.arguments
guard args.count >= 3,
      let dx = Double(args[1]),
      let dy = Double(args[2]) else {
    fputs("Usage: cursor <dx> <dy>\n", stderr)
    exit(1)
}

guard let event = CGEvent(source: nil) else { exit(1) }
let pos = event.location
let newX = max(0, pos.x + dx)
let newY = max(0, pos.y + dy)
let newPoint = CGPoint(x: newX, y: newY)

CGEvent(mouseEventSource: nil, mouseType: .mouseMoved, mouseCursorPosition: newPoint, mouseButton: .left)?
    .post(tap: .cghidEventTap)
