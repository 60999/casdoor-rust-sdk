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
pub struct CasdoorInvitation {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,
    pub description: String,
    pub code: String,
    pub organization: String,
    pub application: String,
    pub user: String,
    pub target: String,
    pub target_type: String,
    pub invitee: String,
    pub invitee_type: String,
    pub status: String,
    pub expires_at: i64,
    pub usage_limit: i32,
    pub usage_count: i32,
    pub is_enabled: bool,
    pub is_public: bool,
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
