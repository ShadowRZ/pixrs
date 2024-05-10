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
use serde::de::DeserializeOwned;
use types::WrappedResponse;

pub use crate::error::Error;
pub use crate::types::*;

/// A `Result` alias where the `Err` case is `pixrs::Error`.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The client to send Pixiv API requests.
pub struct PixivClient {
    client: Client,
    #[allow(dead_code)] // TODO For POST requests
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

    async fn _common_get<T: DeserializeOwned>(&self, url: impl reqwest::IntoUrl) -> Result<T> {
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<WrappedResponse<T>>()
            .await?
            .into()
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
        self._common_get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}?full=1"))
            .await
    }

    /// Get the top works of an user.
    pub async fn user_top_works(&self, user_id: i32) -> Result<UserTopWorks> {
        self._common_get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}/profile/top"))
            .await
    }

    /// Get all the works of an user.
    pub async fn user_all_works(&self, user_id: i32) -> Result<UserAllWorks> {
        self._common_get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}/profile/all"))
            .await
    }

    /// Get the info of an illust.
    pub async fn illust_info(&self, illust_id: i32) -> Result<IllustInfo> {
        self._common_get(format!("{BASE_URL_HTTPS}/ajax/illust/{illust_id}"))
            .await
    }

    /// Get the info of an illust.
    pub async fn illust_pages(&self, illust_id: i32) -> Result<Vec<IllustImage>> {
        self._common_get(format!("{BASE_URL_HTTPS}/ajax/illust/{illust_id}/pages"))
            .await
    }

    /// Get the Pixiv ranking.
    pub async fn ranking(
        &self,
        mode: RankingMode,
        content: RankingContent,
        date: Option<String>,
        page: Option<i32>,
    ) -> Result<PixivRanking> {
        let mode = match mode {
            RankingMode::Daily => "&mode=daily",
            RankingMode::Weekly => "&mode=weekly",
            RankingMode::Monthly => "&mode=monthly",
            RankingMode::Rookie => "&mode=rookie",
            RankingMode::Original => "&mode=original",
            RankingMode::Male => "&mode=male",
            RankingMode::Female => "&mode=female",
            RankingMode::DailyR18 => "&mode=daily_r18",
            RankingMode::WeeklyR18 => "&mode=weekly_r18",
            RankingMode::MaleR18 => "&mode=male_r18",
            RankingMode::FemaleR18 => "&mode=female_r18",
            RankingMode::R18G => "&mode=r18g",
        };
        let content = match content {
            RankingContent::All => "",
            RankingContent::Illust => "&content=illust",
            RankingContent::Ugoira => "&content=ugoira",
            RankingContent::Manga => "&content=manga",
        };
        let page = page.map(|p| format!("&p={p}")).unwrap_or_default();
        let date = date.map(|d| format!("&date={d}")).unwrap_or_default();
        Ok(self
            .client
            .get(format!(
                "{BASE_URL_HTTPS}/ranking.php?format=json{mode}{content}{page}{date}",
            ))
            .send()
            .await?
            .error_for_status()?
            .json::<PixivRanking>()
            .await?)
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
