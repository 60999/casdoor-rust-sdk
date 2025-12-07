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

use crate::entity::{CasdoorConfig, CasdoorToken};

enum Op {
    Add,
    Delete,
    Update,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "add-token"),
            Op::Delete => write!(f, "delete-token"),
            Op::Update => write!(f, "update-token"),
        }
    }
}

#[allow(dead_code)]
pub struct TokenService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> TokenService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_tokens(&self) -> Result<Vec<CasdoorToken>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-tokens?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_token(
        &self,
        name: String,
    ) -> Result<CasdoorToken, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-token?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_token(
        &self,
        op: Op,
        token: CasdoorToken,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            token.owner,
            token.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(&token).send().await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_token(
        &self,
        token: CasdoorToken,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_token(Op::Add, token).await
    }

    pub async fn delete_token(
        &self,
        token: CasdoorToken,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_token(Op::Delete, token).await
    }

    pub async fn update_token(
        &self,
        token: CasdoorToken,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_token(Op::Update, token).await
    }
}
