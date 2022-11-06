use crate::{Client, Result};
use chrono::{DateTime, Local};
use serde::Deserialize;
use std::collections::HashMap;

impl Client {
    pub async fn connections(&self) -> Result<Connections> {
        self.get("/rest/system/connections").await
    }
}

#[derive(Debug, Deserialize)]
pub struct Connections {
    pub total: Totals,
    pub connections: HashMap<String, Device>,
}

#[derive(Debug, Deserialize)]
pub struct Totals {
    pub at: DateTime<Local>,
    #[serde(rename = "inBytesTotal")]
    pub downloaded_bytes: u64,
    #[serde(rename = "outBytesTotal")]
    pub uploaded_bytes: u64,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    #[serde(rename = "type")]
    pub kind: String,
    pub address: String,
    #[serde(rename = "clientVersion")]
    pub version: String,

    pub connected: bool,
    pub paused: bool,

    #[serde(rename = "inBytesTotal")]
    pub downloaded_bytes: u64,
    #[serde(rename = "outBytesTotal")]
    pub uploaded_bytes: u64,

    pub at: DateTime<Local>,
    #[serde(rename = "startedAt")]
    pub started_at: DateTime<Local>,
}
