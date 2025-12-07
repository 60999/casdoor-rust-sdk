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

use crate::entity::{CasdoorConfig, CasdoorGroup};

pub struct GroupService<'a> {
    config: &'a CasdoorConfig,
}

enum Op {
    Add,
    Delete,
    Update,
    Upload,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "add-group"),
            Op::Delete => write!(f, "delete-group"),
            Op::Update => write!(f, "update-group"),
            Op::Upload => write!(f, "upload-groups"),
        }
    }
}

#[allow(dead_code)]
impl<'a> GroupService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_groups(&self) -> Result<Vec<CasdoorGroup>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-groups?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_group(
        &self,
        name: String,
    ) -> Result<CasdoorGroup, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-group?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_group(
        &self,
        op: Op,
        group: CasdoorGroup,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            group.owner,
            group.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(&group).send().await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_group(
        &self,
        group: CasdoorGroup,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_group(Op::Add, group).await
    }

    pub async fn delete_group(
        &self,
        group: CasdoorGroup,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_group(Op::Delete, group).await
    }

    pub async fn update_group(
        &self,
        group: CasdoorGroup,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_group(Op::Update, group).await
    }

    pub async fn upload_groups(
        &self,
        groups: Vec<CasdoorGroup>,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            Op::Upload,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&groups)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }
}
