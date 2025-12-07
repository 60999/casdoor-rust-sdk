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

use crate::entity::{CasdoorConfig, CasdoorOrganization};

pub struct OrganizationService<'a> {
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
            Op::Add => write!(f, "add-organization"),
            Op::Delete => write!(f, "delete-organization"),
            Op::Update => write!(f, "update-organization"),
        }
    }
}

#[allow(dead_code)]
impl<'a> OrganizationService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_organizations(
        &self,
    ) -> Result<Vec<CasdoorOrganization>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-organizations?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_organization(
        &self,
        name: String,
    ) -> Result<CasdoorOrganization, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-organization?id={}&clientId={}&clientSecret={}",
            self.config.endpoint, name, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_default_application(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-default-application?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_organization_names(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-organization-names?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_organization(
        &self,
        op: Op,
        organization: CasdoorOrganization,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            organization.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&organization)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_organization(
        &self,
        organization: CasdoorOrganization,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_organization(Op::Add, organization).await
    }

    pub async fn delete_organization(
        &self,
        organization: CasdoorOrganization,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_organization(Op::Delete, organization).await
    }

    pub async fn update_organization(
        &self,
        organization: CasdoorOrganization,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_organization(Op::Update, organization).await
    }
}
