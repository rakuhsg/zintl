use crate::window::WindowInitialInfo;
use crate::{
    driver::{
        error::{CreateWindowError, WindowHandlerError, WindowHandlerResult},
        win32::utils::string::StringExt,
    },
    event::{Event, ExitCode, RunMode},
    platform::{PlatformError, PlatformResult},
};
use std::mem::size_of;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::{borrow::BorrowMut, cell::Cell, default, mem, mem::transmute};
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, RegisterClassExW, SetWindowLongPtrW, GWLP_USERDATA, WNDCLASSEXW,
};
use windows::Win32::{
    Foundation::{COLORREF, HMODULE, HWND, LPARAM, LRESULT, WPARAM},
    Graphics::{
        Dwm::{
            DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMSBT_MAINWINDOW,
            DWMWA_SYSTEMBACKDROP_TYPE, DWM_SYSTEMBACKDROP_TYPE,
        },
        Gdi::{CreateSolidBrush, InvalidateRect, UpdateWindow},
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Controls::MARGINS,
        HiDpi::{
            GetDpiForWindow, SetProcessDpiAwarenessContext,
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        },
        WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, LoadCursorW, PeekMessageW,
            PostQuitMessage, RegisterClassW, SetWindowPos, ShowWindow, TranslateMessage,
            CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, IDI_APPLICATION, MSG, PM_REMOVE, SWP_NOACTIVATE,
            SWP_NOMOVE, SWP_NOREDRAW, SWP_NOZORDER, SW_SHOW, WINDOW_EX_STYLE, WM_CREATE,
            WM_DESTROY, WM_PAINT, WM_QUIT, WNDCLASSW, WS_OVERLAPPEDWINDOW,
        },
    },
};

pub struct NativeWindow {
    hwnd: HWND,
}

struct WindowState {}

#[derive(Clone)]
struct WindowUserData {
    sender: Sender<Event>,
    state: WindowState,
}

impl WindowUserData {
    pub fn new(sender: mpsc::Sender<Event>) -> Self {
        Self {
            sender,
            state: WindowState {},
        }
    }
}

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

    let _ud = &*(ud);

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
    pub fn new(
        info: WindowInitialInfo,
        hinstance: HMODULE,
        classname: PCWSTR,
        userdata: Arc<RwLock<WindowUserData>>,
    ) -> WindowHandlerResult<Self> {
        let hwnd = match Self::create_window(&info, hinstance, classname, userdata) {
            Ok(hwnd) => hwnd,
            Err(err) => return Err(WindowHandlerError::CreateWindowError(err)),
        };

        Ok(Self { hwnd })
    }

    fn create_window(
        info: &WindowInitialInfo,
        hinstance: HMODULE,
        classname: PCWSTR,
        userdata: Arc<RwLock<WindowUserData>>,
    ) -> Result<HWND, CreateWindowError> {
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
                return Err(CreateWindowError::FailedToCreateWindow);
            }
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
        // Applying Mica window backdrop.
        let margin = MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyBottomHeight: -1,
            cyTopHeight: -1,
        };

        let _ = unsafe { DwmExtendFrameIntoClientArea(self.hwnd, &margin) };

        let mut backdrop = DWMSBT_MAINWINDOW;
        let _ = unsafe {
            DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_SYSTEMBACKDROP_TYPE,
                &mut backdrop as *mut _ as _,
                size_of::<DWM_SYSTEMBACKDROP_TYPE>() as _,
            )
        };
    }

    // TODO: Add feature flag
    // TODO: Move API to impl Platform
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

pub enum PlatformImplError {
    APICallingFailed(String),
}

pub type PlatformImplResult<T> = Result<T, PlatformImplError>;

pub(crate) struct PlatformImpl {
    ud: Arc<RwLock<WindowUserData>>,
    hinstance: HMODULE,
    classname: PCWSTR,
}

impl PlatformImpl {
    pub fn new(sender: mpsc::Sender<Event>) -> PlatformImplResult<Self> {
        Self::enable_hidpi_support();
        let hinstance = Self::get_instance()?;
        let classname = Self::initialize_window_class(hinstance)?;
        let userdata = WindowUserData::new(sender);
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
            Err(e) => Err(
                PlatformImplError::APICallingFailed(format!(
                    "failed to get hinstance: {}",
                    e.message()
                )),
            ),
        }
    }

    fn initialize_window_class(hinstance: HMODULE) {
        let classname = String::from("appwindow").to_pcwstr();

        // SAFETY: IDI_APPLICATION is supported on almost every Windows version.
        // CreateSolidBrush(COLORREF(0x000000)) is safe absolutely.
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

        // SAFETY:
        assert_ne!(
            unsafe { RegisterClassExW(&class) },
            0,
            "RegisterClassExW returns 0"
        );
    }

    fn create_window(&mut self, info: WindowInitialInfo) -> Window {
        let hinstance = self.hinstance;
        let classname = self.classname;
        let ud = self.ud;
        let handle = NativeWindow::new(info, hinstance, classname, ud.clone());
        match handle {
            Ok(handle) => 
        Window { handle },
            Err(
    }
}

pub(crate) struct EventDispatcher {
    mode: RunMode,
    receiver: mpsc::Receiver<Event>,
    sender: mpsc::Sender<Event>,
}

impl EventDispatcher {
    pub fn new(mode: RunMode) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            mode,
            receiver,
            sender,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<Event> {
        self.sender.clone()
    }

    pub fn dispatch_events(&self) -> Event {
        let mut msg = MSG::default();

        unsafe {
            if PeekMessageW(msg.borrow_mut(), None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(msg.borrow_mut());
                DispatchMessageW(msg.borrow_mut());

                if msg.message == WM_QUIT {
                    return Event::Exit(ExitCode::Success);
                }
            }
        }
        Event::None
    }
}
