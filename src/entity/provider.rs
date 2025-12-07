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
pub struct CasdoorProvider {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub type_name: String,
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub protocol: String,
    pub api_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub scope: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub token_url: String,
    pub auth_url: String,
    pub userinfo_url: String,
    pub jwks_url: String,
    pub issuer: String,
    pub certificate: String,
    pub certificate_url: String,
    pub sso_url: String,
    pub sso_issuer: String,
    pub sso_entity_id: String,
    pub sso_metadata_url: String,
    pub sso_metadata_content: String,
    pub sso_name_id_format: String,
    pub sso_binding_type: String,
    pub sso_relay_state: String,
    pub radius_server: String,
    pub radius_secret: String,
    pub radius_listen: String,
    pub ldap_base_dn: String,
    pub ldap_filter: String,
    pub ldap_attribute_map: serde_json::Value,
    pub ldap_bind_dn: String,
    pub ldap_bind_password: String,
    pub ldap_user_identifier: String,
    pub ldap_sync_interval: i32,
    pub email_config: serde_json::Value,
    pub sms_config: serde_json::Value,
    pub storage_config: serde_json::Value,
    pub oauth_config: serde_json::Value,
    pub saml_config: serde_json::Value,
    pub oidc_config: serde_json::Value,
    pub radius_config: serde_json::Value,
    pub ldap_config: serde_json::Value,
    pub webauthn_config: serde_json::Value,
    pub mfa_config: serde_json::Value,
    pub is_enabled: bool,
    pub is_global: bool,
    pub is_default: bool,
    pub is_deletable: bool,
    pub is_editable: bool,
    pub is_sharable: bool,
    pub share_policy: String,
    pub share_expire_in: i32,
    pub share_count: i32,
    pub creator: String,
    pub modifier: String,
    pub approver: String,
    pub approval_state: String,
    pub approved_time: String,
    pub request_id: String,
    pub conditions: serde_json::Value,
    pub metadata: serde_json::Value,
}
