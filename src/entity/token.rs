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
pub struct CasdoorToken {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub client_id: String,
    pub user: String,
    pub code: String,
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
    pub redirect_uri: String,
    pub nonce: String,
    pub session_state: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
    pub is_revoked: bool,
    pub revoked_time: String,
    pub data: serde_json::Value,
}
