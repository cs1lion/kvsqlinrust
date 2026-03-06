use std::array::TryFromSliceError;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;
use std::string::FromUtf8Error;
//use bincode::ErrorKind;
use std::sync::PoisonError;
use bincode::error::{DecodeError, EncodeError};
use serde::{de, ser};

//自定义Result类型
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Parse(String),
    Internal(String),
    WriteConflict,
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::Parse(value.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(value: std::num::ParseFloatError) -> Self {
        Error::Parse(value.to_string())
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Error::Internal(value.to_string())
    }
}
impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::Internal(value.to_string())
    }
}
impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Error::Internal(value.to_string())
    }
}

impl From<EncodeError>for Error{
    fn from(value: EncodeError) -> Self {
        Error::Internal(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Internal(value.to_string())
    }
}
impl std::error::Error for Error {}

impl From<TryFromSliceError> for Error {
    fn from(value: TryFromSliceError) -> Self {
        Error::Internal(value.to_string())
    }
}
impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Internal(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Internal(msg.to_string())
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::WriteConflict => write!(f, "Write conflict, try transaction"),
        }
    }
}