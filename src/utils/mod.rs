use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AOCError {}

impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl<T: Error> From<T> for AOCError {
    fn from(_: T) -> Self {
        AOCError {}
    }
}