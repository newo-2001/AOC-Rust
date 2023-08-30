use std::fmt::{Debug, Display};

use nom::error::VerboseError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Encountered invalid token: `{0}`")]
pub struct InvalidTokenError<T>(pub T) where T: Display + Debug;

pub type ParseError<'a> = nom::Err<VerboseError<&'a str>>;