use dough_core::Type;
use crate::{Error, Result};

pub(super) fn expect(expected: Type, found: Type) -> Result<()> {
    if expected == found {
        Ok(())
    } else {
        Err(Error::TypeMismatch { expected, found })
    }
}
