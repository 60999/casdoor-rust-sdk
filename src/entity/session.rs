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
pub struct CasdoorSession {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub client_id: String,
    pub user: String,
    pub device: String,
    pub ip: String,
    pub last_active_time: String,
    pub expires_at: String,
    pub is_global: bool,
    pub is_online: bool,
    pub data: serde_json::Value,
    pub tags: Vec<String>,
}
