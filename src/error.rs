use std::env::VarError;
use std::io::ErrorKind;
use std::process::exit;

#[derive(Debug)]
pub enum ErrorType {
    Missing,
    Error(String),
}

#[derive(Debug)]
pub struct FetchInfosError(pub ErrorType);

impl FetchInfosError {
    pub fn missing() -> Self {
        Self(ErrorType::Missing)
    }

    pub fn error<S: Into<String>>(error: S) -> Self {
        let error: String = error.into();
        Self(ErrorType::Error(error))
    }

    pub fn error_exit<S: Into<String>>(error: S) -> Self {
        let error: String = error.into();
        println!("An error occurred: {}", error);
        exit(9);
    }
}

impl From<VarError> for FetchInfosError {
    fn from(_: VarError) -> Self {
        FetchInfosError::missing()
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl From<dbus::Error> for FetchInfosError {
    fn from(error: dbus::Error) -> Self {
        FetchInfosError::error(error.to_string())
    }
}

impl From<std::io::Error> for FetchInfosError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound | ErrorKind::PermissionDenied => FetchInfosError::missing(),
            _ => FetchInfosError::error(error.to_string()),
        }
    }
}
