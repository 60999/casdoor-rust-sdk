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

/// OrderService provides order related operations.
#[derive(Debug)]
pub struct OrderService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> OrderService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        OrderService { config }
    }

    /// Get all orders
    pub async fn get_orders(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-orders?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    /// Get an order by id
    pub async fn get_order(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-order?id={}&clientId={}&clientSecret={}",
            self.config.endpoint, id, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// Create an order
    pub async fn create_order(
        &self,
        order: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/add-order?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(order).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Update an order
    pub async fn update_order(
        &self,
        order: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/update-order?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(order).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Delete an order
    pub async fn delete_order(
        &self,
        order: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/delete-order?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(order).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Get orders by owner
    pub async fn get_orders_by_owner(
        &self,
        owner: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-orders-by-owner?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint, owner, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    /// Get orders by user
    pub async fn get_orders_by_user(
        &self,
        user: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-orders-by-user?user={}&clientId={}&clientSecret={}",
            self.config.endpoint, user, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    /// Get orders by product
    pub async fn get_orders_by_product(
        &self,
        product: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-orders-by-product?product={}&clientId={}&clientSecret={}",
            self.config.endpoint, product, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }
}
