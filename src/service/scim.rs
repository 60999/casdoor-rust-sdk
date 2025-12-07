// Copyright 2025 The Casdoor Authors. All Rights Reserved.
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

/// ScimService provides SCIM (System for Cross-domain Identity Management) related operations.
#[derive(Debug)]
pub struct ScimService<'a>
{
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> ScimService<'a>
{
    pub fn new(config: &'a CasdoorConfig) -> Self
    {
        ScimService { config }
    }

    /// SCIM: Get Service Provider Configuration
    pub async fn scim_get_service_provider_config(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/.well-known/scim-configuration?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// SCIM: Get Resource Types
    pub async fn scim_get_resource_types(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/ResourceTypes?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// SCIM: Get Schemas
    pub async fn scim_get_schemas(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Schemas?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// SCIM: Get Users
    pub async fn scim_get_users(
        &self,
        filter: Option<&str>,
        start_index: Option<i32>,
        count: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/scim/Users?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(filter) = filter {
            url.push_str(&format!("&filter={}", filter));
        }
        if let Some(start_index) = start_index {
            url.push_str(&format!("&startIndex={}", start_index));
        }
        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// SCIM: Get User by ID
    pub async fn scim_get_user(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Users/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// SCIM: Create User
    pub async fn scim_create_user(
        &self,
        user: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Users?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(user).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// SCIM: Update User
    pub async fn scim_update_user(
        &self,
        id: &str,
        user: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Users/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().put(url).json(user).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// SCIM: Patch User
    pub async fn scim_patch_user(
        &self,
        id: &str,
        patch: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Users/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().patch(url).json(patch).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// SCIM: Delete User
    pub async fn scim_delete_user(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Users/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().delete(url).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// SCIM: Get Groups
    pub async fn scim_get_groups(
        &self,
        filter: Option<&str>,
        start_index: Option<i32>,
        count: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/scim/Groups?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(filter) = filter {
            url.push_str(&format!("&filter={}", filter));
        }
        if let Some(start_index) = start_index {
            url.push_str(&format!("&startIndex={}", start_index));
        }
        if let Some(count) = count {
            url.push_str(&format!("&count={}", count));
        }

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// SCIM: Get Group by ID
    pub async fn scim_get_group(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Groups/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// SCIM: Create Group
    pub async fn scim_create_group(
        &self,
        group: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Groups?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(group).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// SCIM: Update Group
    pub async fn scim_update_group(
        &self,
        id: &str,
        group: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Groups/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().put(url).json(group).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// SCIM: Patch Group
    pub async fn scim_patch_group(
        &self,
        id: &str,
        patch: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Groups/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().patch(url).json(patch).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// SCIM: Delete Group
    pub async fn scim_delete_group(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/scim/Groups/{}?clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().delete(url).send().await?;
        let json = res.json().await?;
        Ok(json)
    }
}
