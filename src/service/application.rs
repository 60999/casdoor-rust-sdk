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

use crate::entity::{CasdoorApplication, CasdoorConfig};

pub struct ApplicationService<'a> {
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
            Op::Add => write!(f, "add-application"),
            Op::Delete => write!(f, "delete-application"),
            Op::Update => write!(f, "update-application"),
        }
    }
}

#[allow(dead_code)]
impl<'a> ApplicationService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_applications(
        &self,
    ) -> Result<Vec<CasdoorApplication>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-applications?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_application(
        &self,
        name: String,
    ) -> Result<CasdoorApplication, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-application?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_user_application(
        &self,
    ) -> Result<CasdoorApplication, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-user-application?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_organization_applications(
        &self,
    ) -> Result<Vec<CasdoorApplication>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-organization-applications?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    async fn modify_application(
        &self,
        op: Op,
        application: CasdoorApplication,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/{}?id={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            op,
            application.owner,
            application.name,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(&application)
            .send()
            .await?;
        let status = res.status();
        Ok(status)
    }

    pub async fn add_application(
        &self,
        application: CasdoorApplication,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_application(Op::Add, application).await
    }

    pub async fn delete_application(
        &self,
        application: CasdoorApplication,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_application(Op::Delete, application).await
    }

    pub async fn update_application(
        &self,
        application: CasdoorApplication,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        self.modify_application(Op::Update, application).await
    }
}
