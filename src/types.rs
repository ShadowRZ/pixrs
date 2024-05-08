//! Types for the API.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct WrappedResponse<T> {
    pub error: bool,
    pub message: String,
    pub body: T,
}

impl<T> Into<crate::Result<T>> for WrappedResponse<T> {
    fn into(self) -> crate::Result<T> {
        if self.error {
            Result::Err(crate::Error::PixivError(self.message))
        } else {
            Result::Ok(self.body)
        }
        
    }
}