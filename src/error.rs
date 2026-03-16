//! Crate-level error type.

/// All errors that can occur while running scampii animation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ScampiiError {
    /// An I/O error from the terminal or file system.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
