use crate::{Client, Entity, Result};
use async_trait::async_trait;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::marker::Send;

impl Client {
    pub async fn devices(&self) -> Result<Vec<Device>> {
        let configs: Vec<DeviceConfig> = self.get("/rest/config/devices").await?;
        Ok(configs
            .into_iter()
            .map(|c| Device {
                client: self.clone(),
                config: c,
            })
            .collect())
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Device {
    #[derivative(Debug = "ignore")]
    client: Client,

    pub config: DeviceConfig,
}

#[async_trait]
impl Entity for Device {
    async fn reload_cached_config(&mut self) -> Result<()> {
        self.config = self
            .client
            .get(format!("/rest/config/devices/{}", self.id()))
            .await?;
        Ok(())
    }

    async fn patch_config<C: serde::Serialize + Send + Sync>(&mut self, config: C) -> Result<()> {
        self.client
            .patch(format!("/rest/config/devices/{}", self.id()), config)
            .await?;

        self.reload_cached_config().await
    }

    fn id(&self) -> String {
        self.config
            .id
            .as_ref()
            .expect("Config cache should contain an ID")
            .to_string()
    }

    fn name(&self) -> String {
        self.config
            .name
            .as_ref()
            .expect("Config cache should contain a name")
            .to_string()
    }

    async fn rename<S: Into<String> + Send>(&mut self, name: S) -> Result<()> {
        self.patch_config(DeviceConfig {
            name: Some(name.into()),
            ..Default::default()
        })
        .await
    }

    fn paused(&self) -> bool {
        *self
            .config
            .paused
            .as_ref()
            .expect("Config cache should contain a paused status")
    }

    async fn set_paused(&mut self, paused: bool) -> Result<()> {
        self.patch_config(DeviceConfig {
            paused: Some(paused),
            ..Default::default()
        })
        .await
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct DeviceConfig {
    #[serde(rename = "deviceID")]
    pub id: Option<String>,
    pub name: Option<String>,
    pub paused: Option<bool>,
}
