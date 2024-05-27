use std::env::VarError;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum ErrorType {
    Missing,
    Error(String),
}

#[derive(Debug)]
pub struct FetchInfosError(pub ErrorType);

impl FetchInfosError {
    pub fn new_missing() -> Self {
        Self(ErrorType::Missing)
    }

    pub fn new_error<S: Into<String>>(error: S) -> Self {
        let error: String = error.into();
        Self(ErrorType::Error(error))
    }
}

impl From<VarError> for FetchInfosError {
    fn from(_: VarError) -> Self {
        FetchInfosError::new_missing()
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl From<dbus::Error> for FetchInfosError {
    fn from(error: dbus::Error) -> Self {
        FetchInfosError::new_error(error.to_string())
    }
}

impl From<std::io::Error> for FetchInfosError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound | ErrorKind::PermissionDenied => FetchInfosError::new_missing(),
            _ => FetchInfosError::new_error(error.to_string()),
        }
    }
}
