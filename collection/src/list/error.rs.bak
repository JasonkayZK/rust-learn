use std::{error, fmt};

#[derive(Debug, Clone)]
struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index out of range")
    }
}

impl error::Error for OutOfRangeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
