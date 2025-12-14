use std::sync::{mpsc, Arc, RwLock};

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{COLORREF, HMODULE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{CreateSolidBrush, InvalidateRect},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
            WindowsAndMessaging::{
                DefWindowProcW, GetWindowLongPtrW, LoadCursorW, PostQuitMessage, RegisterClassExW,
                SetWindowLongPtrW, CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, GWLP_USERDATA,
                IDI_APPLICATION, WM_CREATE, WM_DESTROY, WM_LBUTTONDOWN, WM_PAINT, WNDCLASSEXW,
            },
        },
    },
};

use super::window::{NativeWindow, WindowUserData};
use crate::{
    driver::{types::*, win32::utils::string::StringExt},
    event::{Event, MouseInput, WindowEvent},
    window::{Window, WindowInitialInfo},
};

// TODO: safety note
unsafe extern "system" fn wndproc(
    hwnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if u_msg == WM_CREATE {
        let cs = &*(l_param.0 as *const CREATESTRUCTW);
        let ud = cs.lpCreateParams;
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, ud as _);
    }

    // SAFETY: ud will not drop at end of function.
    let ud = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const RwLock<WindowUserData>;
    // SAFETY: Do nothing if user data is null
    if ud.is_null() {
        return DefWindowProcW(hwnd, u_msg, w_param, l_param);
    }

    let ud = &*(ud);

    match u_msg {
        WM_PAINT => {
            let _ = InvalidateRect(hwnd, None, true);
            DefWindowProcW(hwnd, u_msg, w_param, l_param)
        }
        WM_LBUTTONDOWN => {
            let input = MouseInput { pos_x: 0, pos_y: 0 };
            match ud.read() {
                Ok(ud) => match ud.send(Event::WindowEvent(WindowEvent::MouseDown(input))) {
                    Ok(..) => {}
                    Err(_e) => {
                        //TODO
                    }
                },
                Err(..) => {}
            }
            DefWindowProcW(hwnd, u_msg, w_param, l_param)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, u_msg, w_param, l_param),
    }
}

pub(crate) struct PlatformImpl {
    ud: Arc<RwLock<WindowUserData>>,
    hinstance: HMODULE,
    classname: PCWSTR,
}

impl PlatformImpl {
    pub fn new(sender: mpsc::Sender<Event>) -> PlatformImplResult<Self> {
        Self::enable_hidpi_support();
        let hinstance = Self::get_hinstance()?;
        let classname = Self::initialize_window_class(hinstance)?;
        let userdata = WindowUserData::new(sender.clone());
        match sender.send(Event::Init) {
            Ok(..) => {}
            Err(..) => return Err(PlatformImplError::MPSCSenderErr),
        };
        Ok(PlatformImpl {
            ud: Arc::new(RwLock::new(userdata)),
            hinstance,
            classname,
        })
    }

    fn enable_hidpi_support() {
        // SAFETY: It is not critical if it be failed.
        let _ =
            unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
    }

    fn get_hinstance() -> PlatformImplResult<HMODULE> {
        // SAFETY: Not safe for now.
        match unsafe { GetModuleHandleW(None) } {
            Ok(h) => Ok(h),
            Err(e) => Err(PlatformImplError::APICallingFailed(format!(
                "failed to get hinstance: {}",
                e.message()
            ))),
        }
    }

    fn initialize_window_class(hinstance: HMODULE) -> PlatformImplResult<PCWSTR> {
        let classname = String::from("appwindow").to_pcwstr();

        // SAFETY: IDI_APPLICATION is supported on almost every Windows version.
        // CreateSolidBrush(COLORREF(0x000000)) is safe absolutely.
        let class = unsafe {
            WNDCLASSEXW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as _,
                hInstance: hinstance.into(),
                lpszClassName: classname,
                lpszMenuName: PCWSTR::null(),
                hCursor: LoadCursorW(None, IDI_APPLICATION).unwrap(),
                hbrBackground: CreateSolidBrush(COLORREF(0x000000)),
                ..Default::default()
            }
        };

        // SAFETY:
        if unsafe { RegisterClassExW(&class) } == 0 {
            Err(PlatformImplError::FailedToRegisterClass)
        } else {
            Ok(classname)
        }
    }

    pub fn create_window(&mut self, info: WindowInitialInfo) -> PlatformImplResult<Window> {
        let hinstance = self.hinstance;
        let classname = self.classname;
        let ud = self.ud.clone();
        let handler = NativeWindow::new(info, hinstance, classname, ud);
        match handler {
            Ok(handler) => Ok(Window::new(handler)),
            Err(e) => Err(PlatformImplError::WHError(e)),
        }
    }
}
