use crate::{Client, Result};
use chrono::{DateTime, Local};
use serde::Deserialize;
use std::collections::HashMap;

impl Client {
    pub async fn folder_stats(&self) -> Result<FoldersStats> {
        self.get("/rest/stats/folder").await
    }
}

pub type FoldersStats = HashMap<String, FolderStats>;

#[derive(Debug, Deserialize)]
pub struct FolderStats {
    #[serde(rename = "lastScan")]
    pub last_scan: DateTime<Local>,
    #[serde(rename = "lastFile")]
    pub last_file: LastFile,
}

#[derive(Debug, Deserialize)]
pub struct LastFile {
    pub at: DateTime<Local>,
    pub filename: String,
    pub deleted: bool,
}
