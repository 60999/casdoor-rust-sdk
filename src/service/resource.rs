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

use crate::entity::{CasdoorConfig, CasdoorResource};

pub struct ResourceService<'a> {
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
            Op::Add => write!(f, "add-resource"),
            Op::Delete => write!(f, "delete-resource"),
            Op::Update => write!(f, "update-resource"),
            Op::Upload => write!(f, "upload-resource"),
        }
    }
}

#[allow(dead_code)]
impl<'a> ResourceService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_resources(&self) -> Result<Vec<CasdoorResource>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-resources?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_resource(
        &self,
        name: String,
    ) -> Result<CasdoorResource, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-resource?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_resource(
        &self,
        op: Op,
        resource: CasdoorResource,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            resource.owner,
            resource.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&resource)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_resource(
        &self,
        resource: CasdoorResource,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_resource(Op::Add, resource).await
    }

    pub async fn delete_resource(
        &self,
        resource: CasdoorResource,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_resource(Op::Delete, resource).await
    }

    pub async fn update_resource(
        &self,
        resource: CasdoorResource,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_resource(Op::Update, resource).await
    }

    pub async fn upload_resource(
        &self,
        resource: CasdoorResource,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_resource(Op::Upload, resource).await
    }
}
