use std::io::Error as IoError;
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

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use Error::*;
        match self {
            &Http(ref e) => writeln!(f, "Http Error: {}", e),
            &IO(ref e) => writeln!(f, "IO Error: {}", e),
            &Serde(ref e) => writeln!(f, "Serialization Error: {}", e),
            &Fault {
                ref code,
                ref errors,
            } => writeln!(f, "Jira Client Error ({}):\n{:#?}", code, errors),
            &Unauthorized => writeln!(f, "Could not connect to Jira: Unauthorized!"),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;
        match self {
            &Http(ref e) => e.description(),
            &IO(ref e) => e.description(),
            &Serde(ref e) => e.description(),
            &Fault { .. } => "Jira client error",
            &Unauthorized => "Unauthorized",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        use Error::*;
        match self {
            &Http(ref e) => Some(e),
            &IO(ref e) => Some(e),
            &Serde(ref e) => Some(e),
            &Fault { .. } => None,
            &Unauthorized => None,
        }
    }
}
