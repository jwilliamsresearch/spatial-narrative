//! Error types for the spatial-narrative library.

use thiserror::Error;

/// Result type alias for spatial-narrative operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in spatial-narrative operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid latitude value (must be between -90 and 90).
    #[error("invalid latitude {0}: must be between -90 and 90")]
    InvalidLatitude(f64),

    /// Invalid longitude value (must be between -180 and 180).
    #[error("invalid longitude {0}: must be between -180 and 180")]
    InvalidLongitude(f64),

    /// Invalid coordinate pair.
    #[error("invalid coordinates: lat={lat}, lon={lon}")]
    InvalidCoordinates { lat: f64, lon: f64 },

    /// Invalid timestamp format.
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),

    /// Missing required field in builder.
    #[error("missing required field: {0}")]
    MissingField(&'static str),

    /// Event not found.
    #[error("event not found: {0}")]
    EventNotFound(String),

    /// Narrative not found.
    #[error("narrative not found: {0}")]
    NarrativeNotFound(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parsing/serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// CSV parsing error.
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    /// Invalid file format.
    #[error("invalid format: {0}")]
    InvalidFormat(String),

    /// Index error.
    #[error("index error: {0}")]
    IndexError(String),

    /// Graph error.
    #[error("graph error: {0}")]
    GraphError(String),

    /// Analysis error.
    #[error("analysis error: {0}")]
    AnalysisError(String),

    /// Parse error.
    #[error("parse error: {0}")]
    ParseError(String),

    /// Generic error with context.
    #[error("{context}: {source}")]
    WithContext {
        context: String,
        #[source]
        source: Box<Error>,
    },
}

impl Error {
    /// Add context to an error.
    pub fn with_context(self, context: impl Into<String>) -> Self {
        Error::WithContext {
            context: context.into(),
            source: Box::new(self),
        }
    }
}
