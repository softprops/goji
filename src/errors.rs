use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use serde_json::error::Error as SerdeError;
use super::Errors;

/// an enumeration over potential errors
/// that may happen when sending a request to jira
#[derive(Debug)]
pub enum Error {
    /// error associated with http request
    Http(HttpError),
    /// error associated IO
    IO(IoError),
    /// error associated with parsing or serializing
    Serde(SerdeError),
    /// client request errors
    Fault { code: StatusCode, errors: Errors },
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        Error::Serde(error)
    }
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Error {
        Error::Http(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::IO(error)
    }
}
