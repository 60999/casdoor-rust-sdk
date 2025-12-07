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

use std::fmt::Display;

use http::StatusCode;

use crate::entity::{CasdoorAdapter, CasdoorConfig};

pub struct AdapterService<'a> {
    config: &'a CasdoorConfig,
}

enum Op {
    Add,
    Delete,
    Update,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "add-adapter"),
            Op::Delete => write!(f, "delete-adapter"),
            Op::Update => write!(f, "update-adapter"),
        }
    }
}

#[allow(dead_code)]
impl<'a> AdapterService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_adapters(&self) -> Result<Vec<CasdoorAdapter>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-adapters?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_adapter(
        &self,
        name: String,
    ) -> Result<CasdoorAdapter, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-adapter?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_adapter(
        &self,
        op: Op,
        adapter: CasdoorAdapter,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            adapter.owner,
            adapter.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&adapter)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_adapter(
        &self,
        adapter: CasdoorAdapter,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_adapter(Op::Add, adapter).await
    }

    pub async fn delete_adapter(
        &self,
        adapter: CasdoorAdapter,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_adapter(Op::Delete, adapter).await
    }

    pub async fn update_adapter(
        &self,
        adapter: CasdoorAdapter,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_adapter(Op::Update, adapter).await
    }

    // Policy related methods
    pub async fn get_policies(&self) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-policies?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_filtered_policies(
        &self,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-filtered-policies?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let mut body = serde_json::Map::new();
        body.insert(
            "fieldIndex".to_string(),
            serde_json::Value::Number(field_index.into()),
        );
        body.insert(
            "fieldValues".to_string(),
            serde_json::Value::Array(
                field_values
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            ),
        );

        let json = reqwest::Client::new()
            .post(url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn add_policy(
        &self,
        policy: Vec<String>,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/add-policy?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&policy)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn update_policy(
        &self,
        old_policy: Vec<String>,
        new_policy: Vec<String>,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/update-policy?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let body = serde_json::json!({"oldPolicy": old_policy, "newPolicy": new_policy});
        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn remove_policy(
        &self,
        policy: Vec<String>,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/remove-policy?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&policy)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }
}
