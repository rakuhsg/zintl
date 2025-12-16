import AppKit

class WindowController: NSWindowController, NSWindowDelegate {
    
}

class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!
    func applicationDidFinishLaunching(_ notification: Notification) {
        let rect = NSRect(x: 0, y: 0, width: 480, height: 300)
        window = NSWindow(
            contentRect: rect,
            styleMask: [.titled, .closable, .resizable],
            backing: .buffered,
            defer: false
        )

        window.title = "Swift"
        window.center()
        window.makeKeyAndOrderFront(nil)
    }
}

@MainActor
final class AppState {
    static let shared = AppState()
    var delegate: AppDelegate?
}

@_cdecl("create_window")
public func createWindow() {
    Task { @MainActor in
        if NSApp == nil {
            let app = NSApplication.shared
            app.setActivationPolicy(.regular)
        }
        
        if AppState.shared.delegate == nil {
            AppState.shared.delegate = AppDelegate()
        }
        
        let app = NSApplication.shared
        
        app.delegate = AppState.shared.delegate
        app.activate(ignoringOtherApps: true)
        //app.run()
        print("init")
    }
}

@_cdecl("runloop_run")
public func run() {
    RunLoop.current.run(mode: .default, before: Date())
    Task { @MainActor in
        NSApplication.shared.run()
    }
}
