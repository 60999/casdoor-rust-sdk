// Copyright 2022 The Casdoor Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::entity::CasdoorConfig;

#[allow(dead_code)]
pub struct SyncerService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> SyncerService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_syncers(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-syncers?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_syncer(
        &self,
        name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-syncer?id={}&clientId={}&clientSecret={}",
            self.config.endpoint, name, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn add_syncer(
        &self,
        syncer: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/add-syncer", self.config.endpoint);
        let body = serde_json::json!(
            {"syncer": syncer,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn update_syncer(
        &self,
        syncer: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/update-syncer", self.config.endpoint);
        let body = serde_json::json!(
            {"syncer": syncer,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn delete_syncer(
        &self,
        syncer: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/delete-syncer", self.config.endpoint);
        let body = serde_json::json!(
            {"syncer": syncer,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn run_syncer(
        &self,
        name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/run-syncer?id={}&clientId={}&clientSecret={}",
            self.config.endpoint, name, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn test_syncer_db(
        &self,
        syncer: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/test-syncer-db", self.config.endpoint);
        let body = serde_json::json!(
            {"syncer": syncer,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }
}
