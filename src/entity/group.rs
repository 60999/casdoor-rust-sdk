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
pub struct CasdoorGroup {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,
    pub avatar: String,
    pub description: String,
    pub parent_id: String,
    pub users: Vec<String>,
    pub roles: Vec<String>,
    pub is_enabled: bool,
    pub type_name: String,
    pub code: String,
    pub members: Vec<String>,
    pub managers: Vec<String>,
    pub is_public: bool,
    pub is_visible: bool,
    pub is_joinable: bool,
    pub is_leaveable: bool,
    pub join_type: String,
    pub leave_type: String,
    pub sync_mode: String,
    pub sync_worker_count: i32,
    pub metadata: serde_json::Value,
}
