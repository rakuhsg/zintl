use crate::driver::win32::types::*;
use crate::driver::win32::utils::string::StringExt;
use crate::event::Event;
use crate::window::WindowInitialInfo;
use std::sync::{mpsc, Arc, RwLock};

use windows::core::PCWSTR;
use windows::Win32::{
    Foundation::{HMODULE, HWND},
    Graphics::{
        Dwm::{
            DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMSBT_MAINWINDOW,
            DWMWA_SYSTEMBACKDROP_TYPE, DWM_SYSTEMBACKDROP_TYPE,
        },
        Gdi::UpdateWindow,
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Controls::MARGINS,
        HiDpi::GetDpiForWindow,
        WindowsAndMessaging::{
            CreateWindowExW, SetWindowPos, SetWindowTextW, ShowWindow, SWP_NOACTIVATE, SWP_NOMOVE,
            SWP_NOREDRAW, SWP_NOZORDER, SW_SHOW, WINDOW_EX_STYLE, WS_OVERLAPPEDWINDOW,
        },
    },
};

pub struct NativeWindow {
    hwnd: HWND,
}

#[derive(Clone)]
struct WindowState {}

#[derive(Clone)]
pub struct WindowUserData {
    sender: mpsc::Sender<Event>,
    state: WindowState,
}

impl WindowUserData {
    pub fn new(sender: mpsc::Sender<Event>) -> Self {
        Self {
            sender,
            state: WindowState {},
        }
    }

    pub fn send(&self, event: Event) -> WindowUDResult<()> {
        self.sender.send(event)?;
        Ok(())
    }
}

impl NativeWindow {
    pub fn new(
        info: WindowInitialInfo,
        hinstance: HMODULE,
        classname: PCWSTR,
        userdata: Arc<RwLock<WindowUserData>>,
    ) -> WHImplResult<Self> {
        let hwnd = match Self::create_window(&info, hinstance, classname, userdata) {
            Ok(hwnd) => hwnd,
            Err(err) => return Err(WHImplError::CreateWindowError(err)),
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

    pub fn set_title(&self, title: String) {
        unsafe {
            SetWindowTextW(self.hwnd, title.to_pcwstr());
        }
    }

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
