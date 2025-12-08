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

use http::StatusCode;

use crate::entity::{CasdoorConfig, CasdoorUser};

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, TokenResponse, TokenUrl};

pub struct AuthService<'a> {
    config: &'a CasdoorConfig,
}

#[allow(dead_code)]
impl<'a> AuthService<'a> {
    pub fn new(config: &'a CasdoorConfig) -> Self {
        Self { config }
    }

    pub fn get_auth_token(&self, code: String) -> Result<String, Box<dyn std::error::Error>> {
        let client_id = ClientId::new(self.config.client_id.clone());
        let client_secret = ClientSecret::new(self.config.client_secret.clone());
        let auth_url = AuthUrl::new(format!(
            "{}/api/login/oauth/authorize",
            self.config.endpoint
        ))?;
        let token_url = TokenUrl::new(format!(
            "{}/api/login/oauth/access_token",
            self.config.endpoint
        ))?;
        let code = AuthorizationCode::new(code);

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url));
        let token_res = client.exchange_code(code).request(http_client)?;

        Ok(token_res.access_token().secret().to_string())
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/login", self.config.endpoint);
        let body = serde_json::json!({
            "owner": self.config.org_name,
            "username": username,
            "password": password,
            "clientId": self.config.client_id,
            "clientSecret": self.config.client_secret
        });

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn signup(
        &self,
        username: String,
        password: String,
        email: String,
        phone: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/signup", self.config.endpoint);
        let body = serde_json::json!({
            "owner": self.config.org_name,
            "username": username,
            "password": password,
            "email": email,
            "phone": phone,
            "clientId": self.config.client_id,
            "clientSecret": self.config.client_secret
        });

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn logout(&self, token: String) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/logout?access_token={}&clientId={}&clientSecret={}",
            self.config.endpoint, token, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().get(url).send().await?;
        Ok(res.status())
    }

    pub async fn sso_logout(
        &self,
        token: String,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/sso-logout?access_token={}&clientId={}&clientSecret={}",
            self.config.endpoint, token, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().get(url).send().await?;
        Ok(res.status())
    }

    pub async fn get_account(
        &self,
        token: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-account?access_token={}&clientId={}&clientSecret={}",
            self.config.endpoint, token, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_userinfo(
        &self,
        token: String,
    ) -> Result<CasdoorUser, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/userinfo?access_token={}&clientId={}&clientSecret={}",
            self.config.endpoint, token, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub fn parse_jwt_token(
        &self,
        token: String,
    ) -> Result<CasdoorUser, Box<dyn std::error::Error>> {
        let res = jsonwebtoken::decode::<CasdoorUser>(
            &token,
            &DecodingKey::from_rsa_pem(self.config.certificate.as_bytes())?,
            &Validation::new(Algorithm::RS256),
        )?;

        Ok(res.claims)
    }

    pub fn get_signin_url(&self, redirect_url: String) -> String {
        let scope = "read";
        let state = self.config.app_name.clone().unwrap_or_default();
        format!("{}/login/oauth/authorize?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}", 
            self.config.endpoint,
            self.config.client_id,
            urlencoding::encode(&redirect_url).into_owned(),
            scope, state)
    }

    pub fn get_signup_url(&self, redirect_url: String) -> String {
        redirect_url.replace("/login/oauth/authorize", "/signup/oauth/authorize")
    }

    pub fn get_signup_url_enable_password(&self) -> String {
        format!(
            "{}/signup/{}",
            self.config.endpoint,
            self.config.app_name.clone().unwrap_or_default()
        )
    }

    pub fn get_user_profile_url(&self, uname: String, token: Option<String>) -> String {
        let param = match token {
            Some(token) if !token.is_empty() => format!("?access_token={}", token),
            _ => "".to_string(),
        };
        format!(
            "{}/users/{}/{}{}",
            self.config.endpoint, self.config.org_name, uname, param
        )
    }

    pub fn get_my_profile_url(&self, token: Option<String>) -> String {
        let param = match token {
            Some(token) if !token.is_empty() => format!("?access_token={}", token),
            _ => "".to_string(),
        };
        format!("{}/account{}", self.config.endpoint, param)
    }

    pub async fn get_app_login(
        &self,
        app_name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-app-login?appName={}&clientId={}&clientSecret={}",
            self.config.endpoint, app_name, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_dashboard(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-dashboard?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn unlink(
        &self,
        provider: String,
        token: String,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/unlink?provider={}&access_token={}&clientId={}&clientSecret={}",
            self.config.endpoint, provider, token, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().post(url).send().await?;
        Ok(res.status())
    }

    pub async fn get_saml_login(
        &self,
        app_name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-saml-login?appName={}&clientId={}&clientSecret={}",
            self.config.endpoint, app_name, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_saml_metadata(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/saml/metadata?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let res = reqwest::Client::new().get(url).send().await?;
        let text = res.text().await?;
        Ok(text)
    }

    pub async fn get_qrcode(
        &self,
        content: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-qrcode?content={}&clientId={}&clientSecret={}",
            self.config.endpoint, content, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn get_captcha_status(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/get-captcha-status?clientId={}&clientSecret={}",
            self.config.endpoint, self.config.client_id, self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(serde_json::from_value(json)?)
    }

    pub async fn get_oauth_token(
        &self,
        code: String,
        grant_type: String,
        redirect_uri: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/login/oauth/access_token", self.config.endpoint);
        let body = serde_json::json!(
            {"grant_type": grant_type,
             "code": code,
             "redirect_uri": redirect_uri,
             "client_id": self.config.client_id,
             "client_secret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn refresh_token(
        &self,
        refresh_token: String,
        grant_type: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/login/oauth/refresh_token", self.config.endpoint);
        let body = serde_json::json!(
            {"grant_type": grant_type,
             "refresh_token": refresh_token,
             "client_id": self.config.client_id,
             "client_secret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn introspect_token(
        &self,
        token: String,
        token_type_hint: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/login/oauth/introspect", self.config.endpoint);
        let body = serde_json::json!(
            {"token": token,
             "token_type_hint": token_type_hint,
             "client_id": self.config.client_id,
             "client_secret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn handle_callback(
        &self,
        code: String,
        state: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/callback", self.config.endpoint);
        let body = serde_json::json!(
            {"code": code,
             "state": state,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn device_auth(
        &self,
        device_code: String,
        user_code: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/device-auth", self.config.endpoint);
        let body = serde_json::json!(
            {"device_code": device_code,
             "user_code": user_code,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    // WebAuthn related APIs
    pub async fn webauthn_signup_begin(
        &self,
        user: String,
        display_name: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/webauthn/signup/begin?userId={}/{}&displayName={}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            user,
            display_name,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn webauthn_signup_finish(
        &self,
        response: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/webauthn/signup/finish", self.config.endpoint);
        let body = serde_json::json!(
            {"response": response,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn webauthn_signin_begin(
        &self,
        user: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/api/webauthn/signin/begin?userId={}/{}&clientId={}&clientSecret={}",
            self.config.endpoint,
            self.config.org_name,
            user,
            self.config.client_id,
            self.config.client_secret
        );

        let json = reqwest::Client::new().get(url).send().await?.json().await?;
        Ok(json)
    }

    pub async fn webauthn_signin_finish(
        &self,
        response: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/webauthn/signin/finish", self.config.endpoint);
        let body = serde_json::json!(
            {"response": response,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    // MFA related APIs
    pub async fn mfa_setup_initiate(
        &self,
        user: String,
        type_: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/mfa/setup/initiate", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "type": type_,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn mfa_setup_verify(
        &self,
        user: String,
        type_: String,
        code: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/mfa/setup/verify", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "type": type_,
             "code": code,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn mfa_setup_enable(
        &self,
        user: String,
        type_: String,
        enabled: bool,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/mfa/setup/enable", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "type": type_,
             "enabled": enabled,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn delete_mfa(
        &self,
        user: String,
        type_: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/delete-mfa", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "type": type_,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn set_preferred_mfa(
        &self,
        user: String,
        type_: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/set-preferred-mfa", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "type": type_,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    // Well-known endpoints
    pub async fn get_openid_configuration(
        &self,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/.well-known/openid-configuration", self.config.endpoint);
        let res = reqwest::Client::new().get(url).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn get_jwks(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/.well-known/jwks.json", self.config.endpoint);
        let res = reqwest::Client::new().get(url).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    // FaceID related APIs
    pub async fn faceid_register(
        &self,
        user: String,
        image: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/faceid/register", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "image": image,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn faceid_verify(
        &self,
        user: String,
        image: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/faceid/verify", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "image": image,
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }

    pub async fn faceid_delete(
        &self,
        user: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/faceid/delete", self.config.endpoint);
        let body = serde_json::json!(
            {"userId": format!("{}/{}", self.config.org_name, user),
             "clientId": self.config.client_id,
             "clientSecret": self.config.client_secret}
        );

        let res = reqwest::Client::new().post(url).json(&body).send().await?;
        let json = res.json().await?;
        Ok(json)
    }
}
