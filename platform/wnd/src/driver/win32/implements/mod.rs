use std::{borrow::BorrowMut, cell::Cell, default, mem, mem::transmute};
use std::mem::size_of;
use std::sync::{Arc, RwLock};
use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM, COLORREF},
    Graphics::{Gdi::{UpdateWindow, CreateSolidBrush, InvalidateRect}, Dwm::{
        DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMSBT_MAINWINDOW, DWMWA_SYSTEMBACKDROP_TYPE, DWM_SYSTEMBACKDROP_TYPE
    },
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Controls::MARGINS,
        HiDpi::{
            GetDpiForWindow, SetProcessDpiAwarenessContext,
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        },
        WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, LoadCursorW,
            PeekMessageW, PostQuitMessage, RegisterClassW, SetWindowPos, ShowWindow,
            TranslateMessage, CS_HREDRAW, CS_VREDRAW, IDI_APPLICATION, MSG, PM_REMOVE,
            SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY,
            WM_QUIT, WNDCLASSW, WS_OVERLAPPEDWINDOW, CREATESTRUCTW, SWP_NOREDRAW, WM_PAINT, WM_CREATE
        },
    },
};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, RegisterClassExW, SetWindowLongPtrW, GWLP_USERDATA, WNDCLASSEXW};
use crate::{
    driver::{
        error::{CreateWindowError, WindowHandlerError, WindowHandlerResult},
        win32::utils::string::StringExt,
    },
    event::{Event, ReturnCode},
};
use crate::window::WindowInitialInfo;

pub struct NativeWindow {
    hwnd: HWND,
}

struct WindowState {

}

struct WindowUserData {
    state: RwLock<WindowState>,
    /// EventRunner is owned by EventLoop
    runner: Arc<EventRunner>,
}

impl WindowUserData {
    pub fn new(runner: Arc<EventRunner>) -> Self {
        Self {
            state: RwLock::new(WindowState {}),
            runner,
        }
    }
}

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

    let ud = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const WindowUserData;
    if ud.is_null() {
        return DefWindowProcW(hwnd, u_msg, w_param, l_param);
    }

    let ud = &*(ud);

    match u_msg {
        WM_PAINT => {
            let _ = InvalidateRect(hwnd, None, true);
            DefWindowProcW(hwnd, u_msg, w_param, l_param)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, u_msg, w_param, l_param),
    }
}

impl NativeWindow {
    pub fn new(runner: Arc<EventRunner>, info: WindowInitialInfo) -> WindowHandlerResult<Self> {
        let hwnd = match Self::create_window(runner, &info) {
            Ok(hwnd) => hwnd,
            Err(err) => return Err(WindowHandlerError::CreateWindowError(err)),
        };

        Ok(Self { hwnd })
    }

    fn create_window(runner: Arc<EventRunner>, info: &WindowInitialInfo) -> Result<HWND, CreateWindowError> {
        let classname = String::from("ryswn").to_pcwstr();
        let hinstance = match unsafe { GetModuleHandleW(None) } {
            Ok(h) => h,
            Err(e) => {
                panic!("fatal: failed to get hinstance: {}", e.message());
            }
        };

        let mut class = unsafe {
            WNDCLASSEXW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                cbSize: mem::size_of::<WNDCLASSEXW>() as _,
                hInstance: hinstance.into(),
                lpszClassName: classname,
                lpszMenuName: PCWSTR::null(),
                hCursor: LoadCursorW(None, IDI_APPLICATION).unwrap(),
                hbrBackground: CreateSolidBrush(COLORREF(0x000000)),
                ..Default::default()
            }
        };

        assert_ne!(unsafe { RegisterClassExW(&class) }, 0, "RegisterClassExW returns 0");

        let userdata = Arc::new(WindowUserData::new(runner.clone()));

        let hwnd = match unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                classname,
                info.title.to_pcwstr(),
                WS_OVERLAPPEDWINDOW,
                info.pos_x,
                info.pos_y,
                1,
                1,
                None,
                None,
                hinstance,
                Some(Arc::into_raw(userdata) as _),
            )
        } {
            Ok(hwnd) => hwnd,
            Err(e) => {
                println!("fatal: {}", e.message());
                return Err(CreateWindowError::FailedToCreateWindow)
            },
        };

        let dpi = unsafe { GetDpiForWindow(hwnd) as f32 };

        match unsafe {
            SetWindowPos(
                hwnd,
                None,
                0,
                0,
                (info.width as f32 * dpi / 96.0) as i32,
                (info.height as f32 * dpi / 96.0) as i32,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_NOREDRAW,
            )
        } {
            Err(..) => return Err(CreateWindowError::UnableToEnableHiDpiSupport),
            _ => {}
        }

        let _ = unsafe { ShowWindow(hwnd, SW_SHOW) };
        let _ = unsafe { UpdateWindow(hwnd) };

        Ok(hwnd)
    }

    pub fn set_title(&self, title: &str) {}

    pub fn get_title(&self) {}

    pub fn apply_system_appearance(&self) {
        let margin = MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyBottomHeight: -1,
            cyTopHeight: -1,
        };

        let _ = unsafe { DwmExtendFrameIntoClientArea(self.hwnd, &margin) };

        let mut backdrop = DWMSBT_MAINWINDOW;
        let _ = unsafe { DwmSetWindowAttribute(self.hwnd, DWMWA_SYSTEMBACKDROP_TYPE, &mut backdrop as *mut _ as _, size_of::<DWM_SYSTEMBACKDROP_TYPE>() as _) };
    }

    pub fn rwh(
        &self,
    ) -> Result<raw_window_handle::RawWindowHandle, raw_window_handle::HandleError> {
        let mut window_handle = raw_window_handle::Win32WindowHandle::new(unsafe {
            std::num::NonZeroIsize::new_unchecked(self.hwnd.0 as _)
        });

        //TODO: Get correct hinstance
        let hinstance = unsafe { GetModuleHandleW(None) }.unwrap();
        window_handle.hinstance = std::num::NonZeroIsize::new(hinstance.0 as _);
        Ok(raw_window_handle::RawWindowHandle::Win32(window_handle))
    }
}

pub(crate) struct EventRunner {
    handler: Cell<Option<Box<dyn FnMut(Event) -> ()>>>,
}

impl EventRunner {
    pub fn new() -> Self {
        Self::enable_hidpi_support();

        Self {
            handler: Cell::new(None),
        }
    }

    fn enable_hidpi_support() {
        let _ = unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
    }

    pub fn register_handler<F: FnMut(Event) -> ()>(&self, handler: F) {
        // Erase lifetime
        let handler =
            unsafe { transmute::<Box<dyn FnMut(Event)>, Box<dyn FnMut(Event)>>(Box::new(handler)) };
        // Resetting an event handler without before clearing is prohibited.
        assert!(self.handler.replace(Some(handler)).is_none());
    }

    pub fn dispatch_events(&self) -> Option<ReturnCode> {
        let mut msg = MSG::default();

        unsafe {
            if PeekMessageW(msg.borrow_mut(), None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(msg.borrow_mut());
                DispatchMessageW(msg.borrow_mut());

                if msg.message == WM_QUIT {
                    return Some(ReturnCode::Exit);
                }
            }
        }
        None
    }
}
