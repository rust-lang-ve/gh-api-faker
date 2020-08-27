use std::fmt;

pub enum Error {
    StaticFileNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::StaticFileNotFound(err) => f.write_str(err),
        }
    }
}
