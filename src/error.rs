use std::error;
use std::fmt;
use std::io;
use std::num;

pub type Res<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FileIO(String),
    ParseError(String),
    RateError(String),
    #[allow(dead_code)]
    General(String),
    LineEmpty,
    LineSeparator,
    LineComment,
}

impl Error {
    fn description(&self) -> &str {
        match *self {
            Error::FileIO(_) => "File IO error",
            Error::ParseError(_) => "Parse Error",
            Error::General(_) => "General Error",
            Error::LineComment => "Line comment",
            Error::LineEmpty => "Line empty",
            Error::LineSeparator => "Line separator",
            Error::RateError(_) => "Rate Error",
        }
    }

    pub fn file_io<S>(error: S) -> Self where S: Into<String> {
        Error::FileIO(error.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FileIO(ref s) => write!(f, "{}: {}", self.description(), s),
            Error::LineComment => write!(f, "{}", self.description()),
            Error::LineEmpty => write!(f, "{}", self.description()),
            Error::LineSeparator => write!(f, "{}", self.description()),
            Error::ParseError(ref s) => write!(f, "{}: {}", self.description(), s),
            Error::General(ref s) => write!(f, "{}: {}", self.description(), s),
            Error::RateError(ref s) => write!(f, "{}: {}", self.description(), s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        Error::description(self)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::FileIO(format!("{}", &error))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(_: num::ParseFloatError) -> Self {
        Error::ParseError("Could not parse number".to_owned())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Error::ParseError("Could not parse number".to_owned())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Error::ParseError(format!("Could not parse date: {}", &e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::RateError(format!("Could not fetch rates: {}", &e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::RateError(format!("Could not parse rates JSON: {}", &e))
    }
}

impl<'a> From<&'a io::Error> for Error {
    fn from(error: &io::Error) -> Self {
        Error::FileIO(format!("{}", error))
    }
}
