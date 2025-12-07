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

use crate::entity::{CasdoorConfig, CasdoorRole};

pub struct RoleService<'a> {
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
            Op::Add => write!(f, "add-role"),
            Op::Delete => write!(f, "delete-role"),
            Op::Update => write!(f, "update-role"),
            Op::Upload => write!(f, "upload-roles"),
        }
    }
}

#[allow(dead_code)]
impl<'a> RoleService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_roles(&self) -> Result<Vec<CasdoorRole>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-roles?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_role(&self, name: String) -> Result<CasdoorRole, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-role?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_role(
        &self,
        op: Op,
        role: CasdoorRole,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            role.owner,
            role.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(&role).send().await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_role(
        &self,
        role: CasdoorRole,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_role(Op::Add, role).await
    }

    pub async fn delete_role(
        &self,
        role: CasdoorRole,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_role(Op::Delete, role).await
    }

    pub async fn update_role(
        &self,
        role: CasdoorRole,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_role(Op::Update, role).await
    }

    pub async fn upload_roles(
        &self,
        roles: Vec<CasdoorRole>,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            Op::Upload,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(&roles).send().await?;
        let status = res.status();
        Ok(status)
    }
}
