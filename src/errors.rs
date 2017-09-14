use std::io::Error as IoError;
use std::error::Error as StdError;
use std::fmt;
use reqwest::Error as HttpError;
use reqwest::StatusCode;
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
    /// invalid credentials
    Unauthorized,
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error{
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::IO(ref e) => e.description(),
            Error::Serde(ref e) => e.description(),
            Error::Fault{ref code, ref errors} => "",
            Error::Unauthorized => "Unauthorized",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Http(ref e) => e.cause(),
            Error::IO(ref e) => e.cause(),
            Error::Serde(ref e) => e.cause(),
            Error::Fault{ref code, ref errors} => None,
            Error::Unauthorized => None,
        }
    }
}

