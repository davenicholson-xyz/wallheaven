use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CustomError {
    APIError,
}

impl Error for CustomError {}
impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::APIError => "Error with API",
        };
        write!(f, "{message}")
    }
}
