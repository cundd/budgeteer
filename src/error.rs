use std::error;
use std::fmt;
use std::io;
use std::num;
use chrono;

pub type Res<T, E = Error> = ::std::result::Result<T, E>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
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

// This is important for other errors to wrap this one.
impl Error {
    fn description(&self) -> &str {
        match self {
            &Error::FileIO(_) => {
                "File IO error"
            }
            &Error::ParseError(_) => {
                "Parse Error"
            }
            &Error::General(_) => {
                "General Error"
            }
            &Error::LineComment => {
                "Line comment"
            }
            &Error::LineEmpty => {
                "Line empty"
            }
            &Error::LineSeparator => {
                "Line separator"
            }
            &Error::RateError(_) => {
                "Rate Error"
            }
        }
    }

    pub fn file_io<S>(error: S) -> Self where S: Into<String> {
        Error::FileIO(error.into())
    }
}


// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::FileIO(ref s) => write!(f, "{}: {}", self.description(), s),
            &Error::LineComment => write!(f, "{}", self.description()),
            &Error::LineEmpty => write!(f, "{}", self.description()),
            &Error::LineSeparator => write!(f, "{}", self.description()),
            &Error::ParseError(ref s) => write!(f, "{}: {}", self.description(), s),
            &Error::General(ref s) => write!(f, "{}: {}", self.description(), s),
            &Error::RateError(ref s) => write!(f, "{}: {}", self.description(), s),
        }
    }
}

// This is important for other errors to wrap this one.
impl error::Error for Error {
    fn description(&self) -> &str {
        Error::description(self)
    }
}

//impl<'a> error::Error for &'a Error {
//    fn description(&self) -> &str {
//        match self {
//            Error::FileIO => {
//                "File IO error"
//            }
//        }
//    }
//}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::FileIO(format!("{}", error::Error::description(&error)))
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
        Error::ParseError(format!("Could not parse date: {}", error::Error::description(&e)))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::RateError(format!("Could not fetch rates: {}", error::Error::description(&e)))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::RateError(format!("Could not parse rates JSON: {}", error::Error::description(&e)))
    }
}

impl<'a> From<&'a io::Error> for Error {
    fn from(error: &io::Error) -> Self {
        Error::FileIO(format!("{}", error))
    }
}
