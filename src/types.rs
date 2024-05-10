//! Types for the API.
#![warn(missing_docs)]

use serde::{de::DeserializeOwned, Deserialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Illust info.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IllustInfo {
    /// The ID of the illust.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,
    /// The title of the illust.
    pub title: String,
    /// The description of the illust in HTML format.
    pub description: String,
    /// The type of the illust.
    pub illust_type: IllustType,
    /// The restriction type for the illust.
    #[serde(rename = "xRestrict")]
    pub restriction: Restriction,
    /// The User ID of the author.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_id: i32,
    /// The name of the author.
    pub user_name: String,
    /// The width of the (first) illust.
    pub width: i32,
    /// The height of the (first) illust.
    pub height: i32,
    // TODO: Date / Time
}

/// Basic profile about a user.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserProfile {
    /// The User ID of the user.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_id: i32,
    /// The name of the user.
    pub name: String,
    /// The profile image.
    pub image: String,
    /// The big variant of profile image.
    pub image_big: String,
    /// Whether this user has an subscription of Pixiv Premium.
    pub premium: bool,
    /// Whether you have followed the user.
    pub is_followed: bool,
    /// Whether this user is in mypixiv.
    pub is_mypixiv: bool,
    /// Whether this user has been blocked.
    pub is_blocking: bool,
    /// The description of the user.
    pub comment: String,
    #[allow(missing_docs)]
    pub followed_back: bool,
    /// Whether this user accept being requested for a work.
    pub accept_request: bool,
}

/// Full info about a user.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserInfo {
    /// The base profile of the user.
    #[serde(flatten)]
    pub profile: UserProfile,
    /// How much user this user is following.
    pub following: i32,
    /// The HTML representation of the user's desription.
    pub comment_html: String,
    #[allow(missing_docs)]
    pub webpage: Option<String>,
    #[allow(missing_docs)]
    pub official: bool,
}

// TODO: Date / Time
/// A basic summary of an illust.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IllustProfile {
    /// The ID of the illust.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,
    /// The title of the illust.
    pub title: String,
    /// The description of the illust in HTML format.
    pub description: String,
    /// The type of the illust.
    pub illust_type: IllustType,
    /// The restriction type for the illust.
    #[serde(rename = "xRestrict")]
    pub restriction: Restriction,
    /// The URL of the first image.
    pub url: String,
    /// The untranslated tags of the illust.
    pub tags: Vec<String>,
    /// The User ID of the author.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_id: i32,
    /// The name of the author.
    pub user_name: String,
    /// The width of the (first) illust.
    pub width: i32,
    /// The height of the (first) illust.
    pub height: i32,
    /// The pages avaliable in the illust.
    pub page_count: i32,
    /// The profile image URL of the author.
    pub profile_image_url: String,
}

/// The top works of an author.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserTopWorks {
    /// The top illusts.
    #[serde(deserialize_with = "crate::de::dict_value_to_vec")]
    pub illusts: Vec<IllustProfile>,
    /// The top illusts.
    #[serde(rename = "manga", deserialize_with = "crate::de::dict_value_to_vec")]
    pub mangas: Vec<IllustProfile>,
    // TODO: Novels
}

/// All the works of an author.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserAllWorks {
    /// The top illusts.
    #[serde(deserialize_with = "crate::de::dict_key_to_vec")]
    pub illusts: Vec<i32>,
    /// The top illusts.
    #[serde(rename = "manga", deserialize_with = "crate::de::dict_key_to_vec")]
    pub mangas: Vec<i32>,
    // TODO: Novels, Manga Series, Novel Series
}

/// An image in a illust.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IllustImage {
    /// The width of the (first) illust.
    pub width: i32,
    /// The height of the (first) illust.
    pub height: i32,
    /// The URLs avaliable in the image.
    pub urls: IllustImageUrls,
}

/// The URLs avaliable in the image.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IllustImageUrls {
    /// The small variant URL of the image.
    pub small: String,
    /// The medium variant URL of the image.
    pub regular: String,
    /// The original variant URL of the image.
    pub original: String,
    // TODO: Thumbs
}

#[allow(missing_docs)]
#[derive(Serialize_repr, Deserialize_repr, Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum IllustType {
    Illustration = 0,
    Manga = 1,
    Animation = 2,
}

#[allow(missing_docs)]
#[derive(Serialize_repr, Deserialize_repr, Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum Restriction {
    General = 0,
    R18 = 1,
    R18G = 2,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(missing_docs)]
pub struct Ranking {
    pub contents: Vec<RankingItem>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(missing_docs)]
pub struct RankingItem {
    pub title: String,
    pub tags: Vec<String>,
    pub user_name: String,
    pub profile_img: String,
    pub illust_id: i32,
    pub user_id: i32,
    pub width: i32,
    pub height: i32,
    pub view_count: i32,
}

/// The ranking mode.
#[allow(missing_docs)]
pub enum RankingMode {
    Daily,
    Weekly,
    Monthly,
    Rookie,
    Original,
    Male,
    Female,
    DailyR18,
    WeeklyR18,
    MaleR18,
    FemaleR18,
    R18G,
}

/// The content in ranking.
#[allow(missing_docs)]
pub enum RankingContent {
    All,
    Illust,
    Ugoira,
    Manga,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct WrappedResponse<T: DeserializeOwned> {
    pub error: bool,
    pub message: String,
    #[serde(deserialize_with = "crate::de::deserialize_err_is_none")]
    pub body: Option<T>,
}

impl<T: DeserializeOwned> From<WrappedResponse<T>> for crate::Result<T> {
    fn from(val: WrappedResponse<T>) -> Self {
        if val.error {
            Result::Err(crate::Error::PixivError(val.message))
        } else {
            Result::Ok(val.body.unwrap())
        }
    }
}
