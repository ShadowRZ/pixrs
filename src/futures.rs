//! Named futures.

use reqwest::Url;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::future::IntoFuture;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::types::WrappedResponse;

/// `IntoFuture` returned by [`crate::PixivClient::get`].
pub struct GetRequest<'a, T: DeserializeOwned> {
    pub(crate) client: &'a reqwest::Client,
    pub(crate) url: Result<Url, reqwest::Error>,

    pub(crate) _type: PhantomData<T>,
}

impl<'a, T: DeserializeOwned> GetRequest<'a, T> {
    /// Specify the language for this request.
    pub fn with_lang(mut self, lang: &str) -> Self {
        match self.url {
            Ok(ref mut url) => {
                {
                    let mut query = url.query_pairs_mut();
                    query.append_pair("lang", lang);
                }
                self
            }
            Err(_) => self,
        }
    }
}

impl<'a, T: DeserializeOwned> IntoFuture for GetRequest<'a, T> {
    type Output = crate::Result<T>;
    type IntoFuture = Pin<Box<dyn Future<Output = crate::Result<T>> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .get(self.url?)
                .send()
                .await?
                .error_for_status()?
                .json::<WrappedResponse<T>>()
                .await?
                .into()
        })
    }
}
