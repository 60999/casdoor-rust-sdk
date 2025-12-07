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
pub struct RecordService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> RecordService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub async fn get_records(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-records?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_records_by_filter(
        &self,
        filter: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/get-records-filter", self.config.endpoint);
        let body = serde_json::json!(
            {"filter": filter,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn add_record(
        &self,
        record: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/add-record", self.config.endpoint);
        let body = serde_json::json!(
            {"record": record,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }
}
