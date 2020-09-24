use std::{
    result::Result as StdResult,
    error::Error as StdError,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    marker::{
        Send,
        Sync
    }
};

use bb8::{
    RunError
};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Unknown,
    CouldntConnect,
    CouldntGetDatabase,
    Custom(Box<dyn StdError>)
}

impl From<RunError<Error>> for Error {
    fn from(err: RunError<Error>) -> Self {
        err.into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#?}", self)
    }
}

impl StdError for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
