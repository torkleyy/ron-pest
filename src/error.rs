use std::result::Result as StdResult;

use pest::error::Error as PestError;

use crate::ast::Rule;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Parsing error")]
    ParseError(#[fail(cause)] PestError<Rule>),
}

impl From<PestError<Rule>> for Error {
    fn from(e: PestError<Rule>) -> Self {
        Error::ParseError(e)
    }
}

pub type Result<T> = StdResult<T, Error>;
