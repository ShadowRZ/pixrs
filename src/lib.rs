//! Implments Rust API to Pixiv.
//!
//! ## Features
//!
//! * `rustls-tls`: Enables the `rustls-tls` feature of reqwest.
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
mod de;
pub mod error;
pub mod futures;
pub mod types;

use std::str::FromStr;
use std::sync::LazyLock;

use async_stream::try_stream;
use futures::GetRequest;
use futures_util::Stream;
use regex::Regex;
use reqwest::{header::HeaderValue, Client};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub use crate::error::Error;
pub use crate::types::*;

/// A `Result` alias where the `Err` case is `pixrs::Error`.
pub type Result<T> = std::result::Result<T, crate::Error>;

/// The client to send Pixiv API requests.
pub struct PixivClient {
    client: Client,
    cookie: HeaderValue,
}

static BASE_URL_HTTPS: &str = "https://www.pixiv.net";
static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

pub(crate) static USER_AGENT_HEADER_VALUE: LazyLock<HeaderValue> =
    LazyLock::new(|| HeaderValue::from_static(USER_AGENT));
pub(crate) static REFERER_HEADER_VALUE: LazyLock<HeaderValue> =
    LazyLock::new(|| HeaderValue::from_static(BASE_URL_HTTPS));

impl PixivClient {
    /// Creates a new client.
    /// ## Argument
    /// * `token`: The session token on your web session. See the [PixivFE guide](https://pixivfe.pages.dev/obtaining-pixivfe-token/) for how to get it.
    pub async fn new(token: &str) -> Result<Self> {
        let cookie = format!("PHPSESSID={token}");
        let mut cookie = HeaderValue::from_str(&cookie)
            .map_err(|_| crate::Error::Other("Cookies data seems to be invaild"))?;
        cookie.set_sensitive(true);
        let client = Client::new();
        Ok(PixivClient { client, cookie })
    }

    /// Creates a new client using an existing [reqwest::Client].
    /// ## Argument
    /// * `token`: The session token on your web session. See the [PixivFE guide](https://pixivfe.pages.dev/obtaining-pixivfe-token/) for how to get it.
    pub async fn from_client(token: &str, client: &reqwest::Client) -> Result<Self> {
        let cookie = format!("PHPSESSID={token}");
        let mut cookie = HeaderValue::from_str(&cookie)
            .map_err(|_| crate::Error::Other("Cookies data seems to be invaild"))?;
        cookie.set_sensitive(true);
        let client = client.clone();
        Ok(PixivClient { client, cookie })
    }

    /// Performs a GET request with Pixiv Web credentials.
    pub fn get<T: DeserializeOwned>(&self, url: impl reqwest::IntoUrl) -> GetRequest<T> {
        let url = url.into_url();
        GetRequest {
            client: &self.client,
            cookie: self.cookie.clone(),
            url,
            _type: PhantomData,
        }
    }

    /// Get the User ID of the logged in user.
    pub async fn self_user_id(&self) -> Result<Option<i32>> {
        let resp = self
            .client
            .get(BASE_URL_HTTPS)
            .header(reqwest::header::COOKIE, self.cookie.clone())
            .header(reqwest::header::REFERER, REFERER_HEADER_VALUE.clone())
            .header(reqwest::header::USER_AGENT, USER_AGENT_HEADER_VALUE.clone())
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
    pub fn user_info(&self, user_id: i32) -> GetRequest<UserInfo> {
        self.get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}?full=1"))
    }

    /// Get the top works of an user.
    pub fn user_top_works(&self, user_id: i32) -> GetRequest<UserTopWorks> {
        self.get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}/profile/top"))
    }

    /// Get all the works of an user.
    pub fn user_all_works(&self, user_id: i32) -> GetRequest<UserAllWorks> {
        self.get(format!("{BASE_URL_HTTPS}/ajax/user/{user_id}/profile/all"))
    }

    /// Get the info of an illust.
    pub fn illust_info(&self, illust_id: i32) -> GetRequest<IllustInfo> {
        self.get(format!("{BASE_URL_HTTPS}/ajax/illust/{illust_id}"))
    }

    /// Get pages of an illust.
    pub fn illust_pages(&self, illust_id: i32) -> GetRequest<Vec<IllustImage>> {
        self.get(format!("{BASE_URL_HTTPS}/ajax/illust/{illust_id}/pages"))
    }

    /// Get the Pixiv ranking.
    pub async fn ranking(
        &self,
        mode: RankingMode,
        content: RankingContent,
        date: Option<String>,
        page: Option<i32>,
    ) -> Result<Ranking> {
        self._ranking(mode, content, &date, page).await
    }

    async fn _ranking(
        &self,
        mode: RankingMode,
        content: RankingContent,
        date: &Option<String>,
        page: Option<i32>,
    ) -> Result<Ranking> {
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
        let date = date
            .as_ref()
            .map(|d| format!("&date={d}"))
            .unwrap_or_default();
        Ok(self
            .client
            .get(format!(
                "{BASE_URL_HTTPS}/ranking.php?format=json{mode}{content}{page}{date}",
            ))
            .header(reqwest::header::COOKIE, self.cookie.clone())
            .header(reqwest::header::REFERER, REFERER_HEADER_VALUE.clone())
            .header(reqwest::header::USER_AGENT, USER_AGENT_HEADER_VALUE.clone())
            .send()
            .await?
            .error_for_status()?
            .json::<Ranking>()
            .await?)
    }

    /// Get the Pixiv ranking as a series of stream.
    pub async fn ranking_stream(
        &self,
        mode: RankingMode,
        content: RankingContent,
        date: Option<String>,
    ) -> impl Stream<Item = Result<RankingItem>> + '_ {
        try_stream! {
            let first = self._ranking(mode, content, &date, None).await?;
            for content in first.contents {
                yield content;
            }
            while let Some(next) = first.next {
                let result = self._ranking(mode, content, &date, Some(next)).await?;
                for content in result.contents {
                    yield content;
                }
            }
        }
    }

    /// Returns the client instance.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    #[allow(dead_code)]
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
