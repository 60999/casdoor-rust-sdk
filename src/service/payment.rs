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

/// PaymentService provides payment related operations.
#[derive(Debug)]
pub struct PaymentService<'a>
{
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> PaymentService<'a>
{
    pub fn new(config: &'a CasdoorConfig) -> Self
    {
        PaymentService { config }
    }

    /// Get all payments
    pub async fn get_payments(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/get-payments?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)  
    }

    /// Get a payment by id
    pub async fn get_payment(&self, id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/get-payment?id={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            id,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    /// Create a payment
    pub async fn create_payment(
        &self,
        payment: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/add-payment?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(payment).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Update a payment
    pub async fn update_payment(
        &self,
        payment: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/update-payment?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(payment).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Delete a payment
    pub async fn delete_payment(
        &self,
        payment: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/delete-payment?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).json(payment).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    /// Get payments by owner
    pub async fn get_payments_by_owner(
        &self,
        owner: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/get-payments-by-owner?owner={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            owner,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)  
    }

    /// Get payments by user
    pub async fn get_payments_by_user(
        &self,
        user: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/get-payments-by-user?user={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            user,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)  
    }

    /// Get payments by order
    pub async fn get_payments_by_order(
        &self,
        order: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/get-payments-by-order?order={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            order,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)  
    }

    /// Process a payment
    pub async fn process_payment(
        &self,
        payment_id: &str,
        action: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/process-payment?id={}&action={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            payment_id,
            action,
            self.config.client_id,
            self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).send().await?;
        let json = res.json().await?;
        Ok(json)
    }
}
