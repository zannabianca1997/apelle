use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct Reporter<E>(pub E);

impl<E> Reporter<E> {
    pub fn new(err: E) -> Self {
        Reporter(err)
    }
}

impl<E> From<E> for Reporter<E> {
    fn from(err: E) -> Self {
        Reporter(err)
    }
}

impl<E: Error> Debug for Reporter<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl<E: Error> Display for Reporter<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)?;

        if let Some(mut source) = self.0.source() {
            write!(f, "\n\nCaused by:\n  - {source}")?;
            while let Some(next_source) = source.source() {
                writeln!(f, "\n  - {next_source}")?;
                source = next_source;
            }
        }

        Ok(())
    }
}
