pub mod devices;
mod folders;
mod stats;
mod system;

use async_trait::async_trait;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error making HTTP request or parsing response")]
    RequestError(#[from] reqwest::Error),
    #[error("Error updating an existing API item")]
    UpdateError,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    base_url: Url,
    token: String,
}

impl Client {
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: Url::parse("http://localhost:8384").expect("Default API URL should be valid"),
            token: token.into(),
        }
    }

    pub fn url(mut self, url: Url) -> Self {
        self.base_url = url;
        self
    }
}

impl Client {
    pub async fn get<S: Into<String>, R: for<'de> serde::Deserialize<'de>>(
        &self,
        path: S,
    ) -> Result<R> {
        Ok(self
            .client
            .get(self.base_url.join(&path.into()).unwrap())
            .header("X-API-Key", &self.token)
            .send()
            .await?
            .json::<R>()
            .await?)
    }

    pub async fn patch<S: Into<String>, B: serde::Serialize>(
        &self,
        path: S,
        item: B,
    ) -> Result<()> {
        let response = self
            .client
            .patch(self.base_url.join(&path.into()).unwrap())
            .header("X-API-Key", &self.token)
            .json(&item)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::UpdateError)
        }
    }
}

#[async_trait]
pub trait Entity {
    async fn reload_cached_config(&mut self) -> Result<()>;
    async fn patch_config<C: serde::Serialize + Send + Sync>(&mut self, config: C) -> Result<()>;

    fn id(&self) -> String;

    fn name(&self) -> String;
    async fn rename<S: Into<String> + Send>(&mut self, name: S) -> Result<()>;

    fn paused(&self) -> bool;
    async fn pause(&mut self) -> Result<()> {
        self.set_paused(true).await
    }
    async fn resume(&mut self) -> Result<()> {
        self.set_paused(false).await
    }
    async fn set_paused(&mut self, paused: bool) -> Result<()>;
}
