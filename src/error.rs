use std::error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn from(message: &str) -> Error {
        Error { message: message.to_string(), }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { self.message.as_ref() }
}
