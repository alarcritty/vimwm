import CoreGraphics
import Foundation

let blockedTypes: [CGEventType] = [
    .leftMouseDown, .leftMouseUp,
    .rightMouseDown, .rightMouseUp,
    .otherMouseDown, .otherMouseUp,
    .leftMouseDragged, .rightMouseDragged, .otherMouseDragged,
    .scrollWheel,
]

let mask = blockedTypes.reduce(CGEventMask(0)) { mask, type_ in
    mask | (1 << type_.rawValue)
}

guard let tap = CGEvent.tapCreate(
    tap: .cghidEventTap,
    place: .headInsertEventTap,
    options: .defaultTap,
    eventsOfInterest: mask,
    callback: { _, _, event, _ in
        return nil
    },
    userInfo: nil
) else {
    fputs("mouseblock: failed to create event tap\n", stderr)
    exit(1)
}

let runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, tap, 0)
CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
CGEvent.tapEnable(tap: tap, enable: true)

let pid = ProcessInfo.processInfo.processIdentifier
try? "\(pid)".write(toFile: "/tmp/vimwm-mouseblock.pid", atomically: true, encoding: .utf8)

signal(SIGTERM) { _ in
    try? FileManager.default.removeItem(atPath: "/tmp/vimwm-mouseblock.pid")
    exit(0)
}
signal(SIGINT) { _ in
    try? FileManager.default.removeItem(atPath: "/tmp/vimwm-mouseblock.pid")
    exit(0)
}

CFRunLoopRun()
