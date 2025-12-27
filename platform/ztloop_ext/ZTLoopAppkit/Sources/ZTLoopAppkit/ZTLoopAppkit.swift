import AppKit
import CoreFoundation

@MainActor
class ZTLoopState {
    static let shared = ZTLoopState()

    var loop: CFRunLoop?;
}

@_cdecl("ztloopa_init")
func ztloopInit() {
    let loop = RunLoop.current.getCFRunLoop();
    Task {@MainActor in
        assert(ZTLoopState.shared.loop == nil, "ZTLoopState is already initialized")
        
        let app = NSApplication.shared
        app.setActivationPolicy(.regular)
        
        ZTLoopState.shared.loop = loop
    }
}

@_cdecl("ztloopa_run")
func ztloopRun() {
    Task { @MainActor in
        guard let loop = ZTLoopState.shared.loop else {
            assertionFailure("ZTLoopState is not initialized")
            return
        }

        CFRunLoopPerformBlock(loop, CFRunLoopMode.commonModes.rawValue) {
            //
        }
        CFRunLoopWakeUp(loop)
    }
    
    RunLoop.current.run()
}

@_cdecl("ztloopa_destroy")
func ztloopDestroy() {
    Task { @MainActor in
        ZTLoopState.shared.loop = nil
    }
}
