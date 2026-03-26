//! Crate-level error type.

/// All errors that can occur while running scampii.
#[derive(Debug)]
#[non_exhaustive]
pub enum ScampiiError {
    /// An I/O error from the terminal or file system.
    Io(std::io::Error),
}

impl std::fmt::Display for ScampiiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for ScampiiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for ScampiiError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
