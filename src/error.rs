use std::fmt::Display;

pub enum Error {
    TaskNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TaskNotFound => write!(f, "Task Not Found"),
        }
    }
}
