//! Implments Rust API to Pixiv.
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
pub mod types;
pub mod error;

pub use crate::error::Error;

/// A `Result` alias where the `Err` case is `pixrs::Error`.
pub type Result<T> = std::result::Result<T, crate::error::Error>;

/// The client to send Pixiv API requests.
pub struct PixivClient {
}