use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct IndexOutOfRangeError;

impl fmt::Display for IndexOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index out of range")
    }
}

impl error::Error for IndexOutOfRangeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
