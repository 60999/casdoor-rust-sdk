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
pub struct CasdoorModel {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub type_name: String,
    pub parent_id: String,
    pub path: String,
    pub tag: String,
    pub content: String,
    pub scope: String,
    pub adapter: String,
    pub storage: String,
    pub is_enabled: bool,
    pub is_public: bool,
    pub is_visible: bool,
    pub is_deletable: bool,
    pub is_editable: bool,
    pub is_downloadable: bool,
    pub is_uploadable: bool,
    pub is_sharable: bool,
    pub share_policy: String,
    pub share_expire_in: i32,
    pub share_count: i32,
    pub download_count: i32,
    pub upload_count: i32,
    pub creator: String,
    pub modifier: String,
    pub approver: String,
    pub approval_state: String,
    pub approved_time: String,
    pub request_id: String,
    pub table: String,
    pub field: String,
    pub policy: String,
    pub conditions: serde_json::Value,
    pub metadata: serde_json::Value,
}
