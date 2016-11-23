use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use serde_json::error::Error as SerdeError;
use super::Errors;

#[derive(Debug)]
pub enum Error {
    Http(HttpError),
    IO(IoError),
    Serde(SerdeError),
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
