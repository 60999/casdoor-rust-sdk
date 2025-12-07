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
pub struct CasdoorAdapter {
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
    pub database: String,
    pub protocol: String,
    pub api_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub config: serde_json::Value,
    pub is_enabled: bool,
    pub is_public: bool,
    pub is_deletable: bool,
    pub is_editable: bool,
    pub creator: String,
    pub modifier: String,
    pub approver: String,
    pub approval_state: String,
    pub approved_time: String,
    pub request_id: String,
    pub metadata: serde_json::Value,
}
