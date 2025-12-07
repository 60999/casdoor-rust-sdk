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
pub struct FormService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> FormService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_global_forms(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-global-forms?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_forms(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-forms?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_form(
        &self,
        name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-form?id={}&clientId={}&clientSecret={}",
            self.config.endpoint, name, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn add_form(
        &self,
        form: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/add-form", self.config.endpoint);
        let body = serde_json::json!(
            {"form": form,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn update_form(
        &self,
        form: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/update-form", self.config.endpoint);
        let body = serde_json::json!(
            {"form": form,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn delete_form(
        &self,
        form: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/delete-form", self.config.endpoint);
        let body = serde_json::json!(
            {"form": form,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }
}
