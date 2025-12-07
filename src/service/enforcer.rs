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

use crate::entity::{CasdoorConfig, CasdoorEnforcer};

enum Op {
    Add,
    Delete,
    Update,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "add-enforcer"),
            Op::Delete => write!(f, "delete-enforcer"),
            Op::Update => write!(f, "update-enforcer"),
        }
    }
}

#[allow(dead_code)]
pub struct EnforcerService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> EnforcerService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_enforcers(&self) -> Result<Vec<CasdoorEnforcer>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-enforcers?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_enforcer(
        &self,
        name: String,
    ) -> Result<CasdoorEnforcer, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-enforcer?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_enforcer(
        &self,
        op: Op,
        enforcer: CasdoorEnforcer,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            enforcer.owner,
            enforcer.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&enforcer)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_enforcer(
        &self,
        enforcer: CasdoorEnforcer,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_enforcer(Op::Add, enforcer).await
    }

    pub async fn delete_enforcer(
        &self,
        enforcer: CasdoorEnforcer,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_enforcer(Op::Delete, enforcer).await
    }

    pub async fn update_enforcer(
        &self,
        enforcer: CasdoorEnforcer,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_enforcer(Op::Update, enforcer).await
    }

    pub async fn enforce(&self, params: Vec<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/enforce?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&params)
            .send()
            .await?;
        let json = res.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn batch_enforce(
        &self,
        params: Vec<Vec<String>>,
    ) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/batch-enforce?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&params)
            .send()
            .await?;
        let json = res.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_all_objects(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-all-objects?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_all_actions(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-all-actions?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_all_roles(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-all-roles?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn run_casbin_command(
        &self,
        command: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/run-casbin-command?command={}&clientId={}&clientSecret={}",
            self.config.endpoint, command, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn refresh_engines(&self) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/refresh-engines?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).send().await?;
        Ok(res.status())
    }
}
