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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CasdoorApplication {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,
    pub avatar: String,
    pub description: String,
    pub homepage_url: String,
    pub favicon_url: String,
    pub slogan: String,
    pub category: String,
    pub language: String,
    pub timezone: String,
    pub country_code2: String,
    pub domain: String,
    pub is_enabled: bool,
    pub enable_password: bool,
    pub enable_sign_up: bool,
    pub enable_sign_in: bool,
    pub enable_email_verification: bool,
    pub enable_phone_verification: bool,
    pub enable_mfa: bool,
    pub default_organization: String,
    pub client_id: String,
    pub client_secret: String,
    pub certificate: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub response_types: Vec<String>,
    pub scope: String,
    pub access_token_expire_in: i32,
    pub refresh_token_expire_in: i32,
    pub id_token_expire_in: i32,
    pub token_format: String,
    pub encrypt_key: String,
    pub jwt_public_key: String,
    pub jwt_private_key: String,
    pub saml_entity_id: String,
    pub saml_metadata_url: String,
    pub saml_metadata_content: String,
    pub oidc_discovery_url: String,
    pub oidc_client_id: String,
    pub oidc_client_secret: String,
    pub oidc_scopes: Vec<String>,
    pub radius_secret: String,
    pub radius_listen: String,
    pub radius_server: String,
    pub token_config: serde_json::Value,
    pub webauthn_config: serde_json::Value,
    pub mfa_config: serde_json::Value,
    pub theme_config: serde_json::Value,
    pub privacy_config: serde_json::Value,
    pub terms_config: serde_json::Value,
    pub report_config: serde_json::Value,
    pub cache_config: serde_json::Value,
    pub audit_log_config: serde_json::Value,
    pub event_config: serde_json::Value,
    pub integration_config: serde_json::Value,
    pub advanced_config: serde_json::Value,
}
