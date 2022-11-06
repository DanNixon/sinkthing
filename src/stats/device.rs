use crate::{Client, Result};
use chrono::{DateTime, Local};
use serde::Deserialize;
use std::collections::HashMap;

impl Client {
    pub async fn device_stats(&self) -> Result<DevicesStats> {
        self.get("/rest/stats/device").await
    }
}

pub type DevicesStats = HashMap<String, DeviceStats>;

#[derive(Debug, Deserialize)]
pub struct DeviceStats {
    #[serde(rename = "lastSeen")]
    pub last_seen: DateTime<Local>,
    #[serde(rename = "lastConnectionDurationS")]
    pub last_connection_duration_seconds: f64,
}
