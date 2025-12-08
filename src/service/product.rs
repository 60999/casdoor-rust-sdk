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

/// ProductService provides product related operations.
#[derive(Debug)]
pub struct ProductService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> ProductService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        ProductService { config }
    }

    /// Get all products
    pub async fn get_products(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-products?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    /// Get a product by id
    pub async fn get_product(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-product?id={}&clientId={}&clientSecret={}",
            self.config.endpoint, id, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// Create a product
    pub async fn create_product(
        &self,
        product: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/add-product?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(product)
            .send()
            .await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Update a product
    pub async fn update_product(
        &self,
        product: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/update-product?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(product)
            .send()
            .await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Delete a product
    pub async fn delete_product(
        &self,
        product: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/delete-product?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new()
            .post(url)
            .json(product)
            .send()
            .await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Get products by owner
    pub async fn get_products_by_owner(
        &self,
        owner: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-products-by-owner?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint, owner, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }
}
