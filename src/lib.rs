//! Implments Rust API to Pixiv.
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
mod de;
pub mod error;
pub mod types;

use std::str::FromStr;

use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use types::WrappedResponse;

pub use crate::error::Error;
pub use crate::types::*;

/// A `Result` alias where the `Err` case is `pixrs::Error`.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The client to send Pixiv API requests.
pub struct PixivClient {
    client: Client,
    csrf_token: String,
}

static BASE_URL_HTTPS: &str = "https://www.pixiv.net";
static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

impl PixivClient {
    /// Creates a new client.
    /// ## Argument
    /// * `token`: The session token on your web session. See the [PixivFE guide](https://pixivfe.pages.dev/obtaining-pixivfe-token/) for how to get it.
    pub async fn new(token: &str) -> Result<Self> {
        let cookie = format!("PHPSESSID={token}");
        let mut headers = HeaderMap::new();
        let mut cookie = HeaderValue::from_str(&cookie)
            .map_err(|_| crate::Error::Other("Cookies data seems to be invaild"))?;
        cookie.set_sensitive(true);
        headers.append(reqwest::header::COOKIE, cookie);
        headers.append(
            reqwest::header::REFERER,
            HeaderValue::from_static(BASE_URL_HTTPS),
        );
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()?;
        let csrf_token = PixivClient::csrf_token(&client).await?;
        Ok(PixivClient { client, csrf_token })
    }

    /// Get the User ID of the logged in user.
    pub async fn self_user_id(&self) -> Result<Option<i32>> {
        let resp = self
            .client
            .get(BASE_URL_HTTPS)
            .send()
            .await?
            .error_for_status()?;
        let headers = resp.headers();
        Ok(headers
            .get("x-userid")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| <i32 as FromStr>::from_str(value).ok()))
    }

    /// Get the info of an user.
    pub async fn user_info(&self, user_id: i32) -> Result<UserInfo> {
        self.client
            .get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}?full=1"))
            .send()
            .await?
            .error_for_status()?
            .json::<WrappedResponse<UserInfo>>()
            .await?
            .into()
    }

    /// Get the top works of an user.
    pub async fn user_top_works(&self, user_id: i32) -> Result<UserTopWorks> {
        self.client
            .get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}/profile/top"))
            .send()
            .await?
            .error_for_status()?
            .json::<WrappedResponse<UserTopWorks>>()
            .await?
            .into()
    }

    /// Get all the works of an user.
    pub async fn user_all_works(&self, user_id: i32) -> Result<UserAllWorks> {
        self.client
            .get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}/profile/all"))
            .send()
            .await?
            .error_for_status()?
            .json::<WrappedResponse<UserAllWorks>>()
            .await?
            .into()
    }

    /// Get the info of an illust.
    pub async fn illust_info(&self, illust_id: i32) -> Result<IllustInfo> {
        self.client
            .get(format!("{BASE_URL_HTTPS}/ajax/illust/{illust_id}"))
            .send()
            .await?
            .error_for_status()?
            .json::<WrappedResponse<IllustInfo>>()
            .await?
            .into()
    }

    /// Get the info of an illust.
    pub async fn illust_pages(&self, illust_id: i32) -> Result<Vec<IllustImage>> {
        self.client
            .get(format!("{BASE_URL_HTTPS}/ajax/illust/{illust_id}/pages"))
            .send()
            .await?
            .error_for_status()?
            .json::<WrappedResponse<Vec<IllustImage>>>()
            .await?
            .into()
    }

    async fn csrf_token(client: &Client) -> Result<String> {
        let resp = client
            .get(BASE_URL_HTTPS)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let re = Regex::new(r#"token":"([^"])"#).unwrap();
        let caps = re
            .captures(&resp)
            .ok_or(crate::Error::Other("No CSRF Token Found"))?;
        let token = &caps[1];
        Ok(token.to_string())
    }
}
