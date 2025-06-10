use std::env::VarError;
use std::fmt::Display;
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
    pub const fn missing() -> Self {
        Self(ErrorType::Missing)
    }

    pub fn error<S: Into<String>>(error: S) -> Self {
        let error: String = error.into();
        Self(ErrorType::Error(error))
    }

    pub fn error_exit<S: Display>(error: S) -> ! {
        println!("{error}");
        exit(9);
    }
}

impl From<VarError> for FetchInfosError {
    fn from(_: VarError) -> Self {
        Self::missing()
    }
}

impl From<std::io::Error> for FetchInfosError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound | ErrorKind::PermissionDenied => Self::missing(),
            _ => Self::error(error.to_string()),
        }
    }
}
