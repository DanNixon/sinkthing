use crate::{Client, Result};
use serde::Deserialize;

impl Client {
    pub async fn ping(&self) -> Result<Pong> {
        self.get("/rest/system/ping").await
    }
}

#[derive(Debug, Deserialize)]
pub struct Pong {
    pub ping: String,
}
