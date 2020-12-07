use std::num::ParseIntError;

use thiserror::Error;
use displaydoc::Display;

#[non_exhaustive]
#[derive(PartialEq, Display, Error, Debug)]
pub enum AocError {
    /// failed to parse: {0}
    ParseError(String),
    /// could not parse int
    InvalidInt(#[from] ParseIntError),
}