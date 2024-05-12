//! Error types.
use thiserror::Error;

/// An error that can occur.
#[derive(Error, Debug)]
pub enum Error {
    /// The error returned when the HTTP status is 200 but Pixiv API specified
    /// that an error occured.
    #[error("Pixiv API Error: {0}")]
    PixivError(String),
    /// HTTP error.
    #[error("HTTP Error")]
    HttpError(#[from] reqwest::Error),
    /// Other error.
    #[error("{0}")]
    Other(&'static str),
}
