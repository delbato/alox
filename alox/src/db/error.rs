use std::{
    fmt::{
        Result as FmtResult,
        Formatter,
        Display
    },
    error::Error as StdError,
    result::Result as StdResult,
    marker::{
        Send,
        Sync
    }
};

#[derive(Debug)]
pub enum Error {
    Unknown,
    CouldntConnect
}

pub type Result<T> = StdResult<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#?}", self)
    }
}

impl StdError for Error  {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
