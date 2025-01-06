use std::error;
use std::fmt;
use std::io;
use std::num;

pub type Res<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FileIO(String),
    Parse(String),
    Rate(String),
    #[allow(dead_code)]
    General(String),
    LineEmpty,
    LineSeparator,
    LineComment,
    Persistence(String),
}

impl Error {
    fn description(&self) -> &str {
        match *self {
            Error::FileIO(_) => "File IO error",
            Error::Parse(_) => "Parse Error",
            Error::General(_) => "General Error",
            Error::LineComment => "Line comment",
            Error::LineEmpty => "Line empty",
            Error::LineSeparator => "Line separator",
            Error::Rate(_) => "Rate Error",
            Error::Persistence(_) => "Persistence Error",
        }
    }

    pub fn file_io<S>(error: S) -> Self
    where
        S: Into<String>,
    {
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
            Error::Parse(ref s) => write!(f, "{}: {}", self.description(), s),
            Error::General(ref s) => write!(f, "{}: {}", self.description(), s),
            Error::Rate(ref s) => write!(f, "{}: {}", self.description(), s),
            Error::Persistence(ref s) => write!(f, "{}: {}", self.description(), s),
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
        Error::Parse("Could not parse number".to_owned())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Error::Parse("Could not parse number".to_owned())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Error::Parse(format!("Could not parse date: {}", &e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Rate(format!("Could not fetch rates: {}", &e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Rate(format!("Could not parse rates JSON: {}", &e))
    }
}

impl From<dialoguer::Error> for Error {
    fn from(e: dialoguer::Error) -> Self {
        Error::General(format!("Input wizard error: {}", &e))
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::Persistence(format!("Persistence error: {}", &e))
    }
}

impl From<std::boxed::Box<dyn sqlx::error::DatabaseError>> for Error {
    fn from(e: std::boxed::Box<dyn sqlx::error::DatabaseError>) -> Self {
        Error::Persistence(format!("Database error: {}", &e.message()))
    }
}

impl From<&dyn sqlx::error::DatabaseError> for Error {
    fn from(e: &dyn sqlx::error::DatabaseError) -> Self {
        Error::Persistence(format!("Database error: {}", &e.message()))
    }
}
// impl<E: sqlx::error::DatabaseError> From<E> for Error {
//     fn from(e: E) -> Self {
//         Error::Persistence(format!("Database error: {}", &e.message()))
//     }
// }

impl From<&io::Error> for Error {
    fn from(error: &io::Error) -> Self {
        Error::FileIO(format!("{}", error))
    }
}
