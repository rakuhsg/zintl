use crate::event::Event;
use crate::platform::PlatformError;
use crate::window::WindowError;
use std::sync::mpsc;

/// Window Handler Implementation Error
#[derive(Debug, Clone)]
pub enum WHImplError {
    CreateWindowError(CreateWindowError),
}

impl From<WHImplError> for WindowError {
    fn from(v: WHImplError) -> Self {
        WindowError::ImplError(v)
    }
}

pub type WHImplResult<T> = Result<T, WHImplError>;

#[derive(Debug, Clone)]
pub enum CreateWindowError {
    FailedToCreateWindow,
    UnableToEnableHiDpiSupport,
    // windows
    UnableToRegisterClass,
}

impl From<CreateWindowError> for WHImplError {
    fn from(v: CreateWindowError) -> Self {
        WHImplError::CreateWindowError(v)
    }
}

#[derive(Debug, Clone)]
pub enum PlatformImplError {
    APICallingFailed(String),
    FailedToRegisterClass,
    WHError(WHImplError),
    MPSCSenderErr,
}

impl From<PlatformImplError> for PlatformError {
    fn from(v: PlatformImplError) -> Self {
        PlatformError::ImplError(v)
    }
}

pub type PlatformImplResult<T> = Result<T, PlatformImplError>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum WindowUDError {
    SendMessageError(mpsc::SendError<Event>),
}

impl From<mpsc::SendError<Event>> for WindowUDError {
    fn from(v: mpsc::SendError<Event>) -> Self {
        WindowUDError::SendMessageError(v)
    }
}

pub type WindowUDResult<T> = Result<T, WindowUDError>;
