#[derive(Debug)]
pub enum WindowHandlerError {
    CreateWindowError(CreateWindowError),
}

pub type WindowHandlerResult<T> = Result<T, WindowHandlerError>;

#[derive(Debug)]
pub enum CreateWindowError {
    FailedToCreateWindow,
    UnableToEnableHiDpiSupport,
    // windows
    UnableToRegisterClass,
}

pub type CreateWindowResult<T> = Result<T, CreateWindowError>;
