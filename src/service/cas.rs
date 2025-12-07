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

use crate::entity::{CasdoorConfig, CasdoorUser};

/// CasService provides CAS (Central Authentication Service) related operations.
#[derive(Debug)]
pub struct CasService<'a>
{
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> CasService<'a>
{
    pub fn new(config: &'a CasdoorConfig) -> Self
    {
        CasService { config }
    }

    /// CAS 1.0: Validate ticket
    pub async fn cas_validate(&self, ticket: &str, service: &str) -> Result<String, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/cas/validate?ticket={}&service={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            ticket,
            service,
            self.config.client_id,
            self.config.client_secret
        );

        let text = reqwest::Client::new().get(url).send().await?.text().await?;
        Ok(text)
    }

    /// CAS 2.0: Service ticket validation
    pub async fn cas_service_validate(
        &self,
        ticket: &str,
        service: &str,
        pgt_url: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/cas/serviceValidate?ticket={}&service={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            ticket,
            service,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(pgt_url) = pgt_url {
            url.push_str(&format!("&pgtUrl={}", pgt_url));
        }

        let text = reqwest::Client::new().get(url).send().await?.text().await?;
        Ok(text)
    }

    /// CAS 2.0: Proxy ticket validation
    pub async fn cas_proxy_validate(
        &self,
        ticket: &str,
        service: &str,
        pgt_url: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/cas/proxyValidate?ticket={}&service={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            ticket,
            service,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(pgt_url) = pgt_url {
            url.push_str(&format!("&pgtUrl={}", pgt_url));
        }

        let text = reqwest::Client::new().get(url).send().await?.text().await?;
        Ok(text)
    }

    /// CAS 3.0: Service ticket validation with attributes
    pub async fn cas3_service_validate(
        &self,
        ticket: &str,
        service: &str,
        pgt_url: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/cas/p3/serviceValidate?ticket={}&service={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            ticket,
            service,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(pgt_url) = pgt_url {
            url.push_str(&format!("&pgtUrl={}", pgt_url));
        }

        let text = reqwest::Client::new().get(url).send().await?.text().await?;
        Ok(text)
    }

    /// CAS 3.0: Proxy ticket validation with attributes
    pub async fn cas3_proxy_validate(
        &self,
        ticket: &str,
        service: &str,
        pgt_url: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/cas/p3/proxyValidate?ticket={}&service={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            ticket,
            service,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(pgt_url) = pgt_url {
            url.push_str(&format!("&pgtUrl={}", pgt_url));
        }

        let text = reqwest::Client::new().get(url).send().await?.text().await?;
        Ok(text)
    }

    /// CAS: Proxy
    pub async fn cas_proxy(
        &self,
        pgt: &str,
        target_service: &str,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/cas/proxy?pgt={}&targetService={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            pgt,
            target_service,
            self.config.client_id,
            self.config.client_secret
        );

        let text = reqwest::Client::new().get(url).send().await?.text().await?;
        Ok(text)
    }

    /// CAS: Get user profile
    pub async fn cas_get_user_profile(
        &self,
        ticket: &str,
        service: &str,
    ) -> Result<CasdoorUser, Box<dyn std::error::Error>>
    {
        let url = format!(
            "{}/api/cas/getUserProfile?ticket={}&service={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            ticket,
            service,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)  
    }

    /// CAS: Login URL
    pub async fn cas_get_login_url(
        &self,
        service: &str,
        renew: Option<bool>,
        gateway: Option<bool>,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/cas/getLoginUrl?service={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            service,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(renew) = renew {
            url.push_str(&format!("&renew={}", renew));
        }
        if let Some(gateway) = gateway {
            url.push_str(&format!("&gateway={}", gateway));
        }

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        let url_str = serde_json::from_value(json)?;
        Ok(url_str)
    }

    /// CAS: Logout URL
    pub async fn cas_get_logout_url(
        &self,
        service: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut url = format!(
            "{}/api/cas/getLogoutUrl?clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.client_id,
            self.config.client_secret
        );

        if let Some(service) = service {
            url.push_str(&format!("&service={}", service));
        }

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        let url_str = serde_json::from_value(json)?;
        Ok(url_str)
    }
}
