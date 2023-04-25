#![macro_use]
use std::error::Error;

// New error type encapsulating the original error and location data.
#[derive(Debug, Clone)]
pub struct LocatedError<E: Error + 'static> {
    pub inner: E,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl<E: Error + 'static> Error for LocatedError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.inner)
    }
}

impl<E: Error + 'static> std::fmt::Display for LocatedError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}:{}:{}",
            self.inner, self.file, self.line, self.column
        )
    }
}

// The core idea: convenience macro to create the structure
macro_rules! loc {
    () => {
        |e| LocatedError {
            inner: e,
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}
