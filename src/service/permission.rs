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

use crate::entity::{CasdoorConfig, CasdoorPermission};

pub struct PermissionService<'a> {
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
            Op::Add => write!(f, "add-permission"),
            Op::Delete => write!(f, "delete-permission"),
            Op::Update => write!(f, "update-permission"),
            Op::Upload => write!(f, "upload-permissions"),
        }
    }
}

#[allow(dead_code)]
impl<'a> PermissionService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_permissions(
        &self,
    ) -> Result<Vec<CasdoorPermission>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-permissions?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_permissions_by_submitter(
        &self,
        submitter: String,
    ) -> Result<Vec<CasdoorPermission>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-permissions-by-submitter?submitter={}&clientId={}&clientSecret={}",
            self.config.endpoint, submitter, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_permissions_by_role(
        &self,
        role: String,
    ) -> Result<Vec<CasdoorPermission>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-permissions-by-role?role={}&clientId={}&clientSecret={}",
            self.config.endpoint, role, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_permission(
        &self,
        name: String,
    ) -> Result<CasdoorPermission, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-permission?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_permission(
        &self,
        op: Op,
        permission: CasdoorPermission,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            permission.owner,
            permission.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&permission)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_permission(
        &self,
        permission: CasdoorPermission,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_permission(Op::Add, permission).await
    }

    pub async fn delete_permission(
        &self,
        permission: CasdoorPermission,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_permission(Op::Delete, permission).await
    }

    pub async fn update_permission(
        &self,
        permission: CasdoorPermission,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_permission(Op::Update, permission).await
    }

    pub async fn upload_permissions(
        &self,
        permissions: Vec<CasdoorPermission>,
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
            .json(&permissions)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }
}
