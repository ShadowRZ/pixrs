//! Implments Rust API to Pixiv.
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
pub mod error;
pub mod types;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use types::{IllustInfo, WrappedResponse};

pub use crate::error::Error;

/// A `Result` alias where the `Err` case is `pixrs::Error`.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The client to send Pixiv API requests.
pub struct PixivClient {
    client: Client,
}

static BASE_URL_HTTPS: &str = "https://www.pixiv.net";
static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

impl PixivClient {
    /// Creates a new client.
    /// ## Argument
    /// * `token`: The session token on your web session. See the [PixivFE guide](https://pixivfe.pages.dev/obtaining-pixivfe-token/) for how to get it.
    pub fn new(token: &str) -> Result<Self> {
        let cookie = format!("PHPSESSID={token}");
        let mut headers = HeaderMap::new();
        let mut cookie = HeaderValue::from_str(&cookie)
            .map_err(|_| crate::Error::Other("Cookies data seems to be invaild"))?;
        cookie.set_sensitive(true);
        headers.append(reqwest::header::COOKIE, cookie);
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()?;
        Ok(PixivClient { client })
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
}
