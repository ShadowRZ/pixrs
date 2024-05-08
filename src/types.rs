//! Types for the API.
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Illust info.
#[derive(Deserialize, Serialize, Debug)]
pub struct IllustInfo {
    /// The ID of the illust.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,
    /// The title of the illust.
    pub title: String,
    /// The description of the illust in HTML format.
    pub description: String,
    /// The type of the illust.
    #[serde(rename = "illustType")]
    pub illust_type: IllustType,
    /// The restriction type for the illust.
    #[serde(rename = "xRestrict")]
    pub restriction: Restriction,
    /// The User ID of the author.
    #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
    pub user_id: i32,
    /// The name of the author.
    #[serde(rename = "userName")]
    pub user_name: String,
    /// The width of the (first) illust.
    pub width: i32,
    /// The height of the (first) illust.
    pub height: i32,
}

#[allow(missing_docs)]
#[derive(Serialize_repr, Deserialize_repr, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum IllustType {
    Illustration = 0,
    Manga = 1,
    Animation = 2,
}

#[allow(missing_docs)]
#[derive(Serialize_repr, Deserialize_repr, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Restriction {
    General = 0,
    R18 = 1,
    R18G = 2,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct WrappedResponse<T> {
    pub error: bool,
    pub message: String,
    pub body: Option<T>,
}

impl<T> From<WrappedResponse<T>> for crate::Result<T> {
    fn from(val: WrappedResponse<T>) -> Self {
        if val.error {
            Result::Err(crate::Error::PixivError(val.message))
        } else {
            Result::Ok(val.body.unwrap())
        }
    }
}
