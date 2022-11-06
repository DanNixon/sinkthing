use crate::{Client, Result};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

impl Client {
    pub async fn folders(&self) -> Result<Vec<FolderConfig>> {
        self.get("/rest/config/folders").await
    }
}

#[derive(Debug, Deserialize)]
pub struct FolderConfig {
    pub id: String,
    pub label: String,
    pub path: PathBuf,
    pub paused: bool,

    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}
