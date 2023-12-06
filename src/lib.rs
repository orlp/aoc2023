use core::panic::Location;
use std::fmt;

#[derive(Debug)]
pub struct NoneError {
    location: &'static Location<'static>,
}

impl std::error::Error for NoneError {}

impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "some() was called on None value at {}", self.location)
    }
}

pub trait OptionSomeExt {
    type Item;

    fn some(self) -> Result<Self::Item, NoneError>;
}

impl<T> OptionSomeExt for Option<T> {
    type Item = T;

    #[track_caller]
    fn some(self) -> Result<Self::Item, NoneError> {
        match self {
            Some(val) => Ok(val),
            None => Err(NoneError {
                location: Location::caller(),
            }),
        }
    }
}
