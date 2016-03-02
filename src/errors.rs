use std::io::Error as IoError;
use hyper::Error as HttpError;
use hyper::status::StatusCode;
use rustc_serialize::json::{DecoderError, EncoderError, ParserError};
use serde_json::error::{Error as SerdeError};

#[derive(Debug)]
pub enum Error {
    Decoding(DecoderError),
    Encoding(EncoderError),
    Parse(ParserError),
    Http(HttpError),
    IO(IoError),
    Serde(SerdeError),
    Fault {
        code: StatusCode,
        message: String,
    },
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        Error::Serde(error)
    }
}

impl From<ParserError> for Error {
    fn from(error: ParserError) -> Error {
        Error::Parse(error)
    }
}

impl From<DecoderError> for Error {
    fn from(error: DecoderError) -> Error {
        Error::Decoding(error)
    }
}

impl From<EncoderError> for Error {
    fn from(error: EncoderError) -> Error {
        Error::Encoding(error)
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
