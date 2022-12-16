use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParsingError;

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Errored while parsing input.")
    }
}

impl std::error::Error for ParsingError {}

impl From<ParseIntError> for ParsingError {
    fn from(_: ParseIntError) -> Self {
        ParsingError
    }
}

pub type Result<T> = std::result::Result<T, ParsingError>;