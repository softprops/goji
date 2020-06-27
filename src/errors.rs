// Third party
use reqwest::Error as HttpError;
use reqwest::StatusCode;
use serde_json::error::Error as SerdeError;
use std::io::Error as IoError;
use url::ParseError;

// Ours
use crate::Errors;

/// An enumeration over potential errors that may
/// happen when sending a request to jira
#[derive(Debug)]
pub enum Error {
    /// Error associated with http request
    Http(HttpError),
    /// Error associated IO
    IO(IoError),
    /// Error associated with parsing or serializing
    Serde(SerdeError),
    /// Client request errors
    Fault { code: StatusCode, errors: Errors },
    /// Invalid credentials
    Unauthorized,
    /// HTTP method is not allowed
    MethodNotAllowed,
    /// Page not found
    NotFound,
    /// URI parse error
    ParseError(ParseError),
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Error {
        Error::ParseError(error)
    }
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
        use crate::Error::*;

        match *self {
            Http(ref e) => writeln!(f, "HTTP Error: {}", e),
            IO(ref e) => writeln!(f, "IO Error: {}", e),
            Serde(ref e) => writeln!(f, "Serialization Error: {}", e),
            Fault {
                ref code,
                ref errors,
            } => writeln!(f, "Jira Client Error ({}):\n{:#?}", code, errors),
            Unauthorized => writeln!(f, "Could not connect to Jira: Unauthorized!"),
            MethodNotAllowed => writeln!(f, "Could not connect to Jira: Method Not Allowed!",),
            NotFound => writeln!(f, "Could not connect to Jira: No Found!"),
            ParseError(ref e) => writeln!(f, "Could not connect to Jira: {:?}!", e),
        }
    }
}

impl ::std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use crate::Error::*;

        match *self {
            Http(ref e) => Some(e),
            IO(ref e) => Some(e),
            Serde(ref e) => Some(e),
            Fault { .. } => None,
            _ => None,
        }
    }
}
