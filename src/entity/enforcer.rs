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
pub struct CasdoorEnforcer {
    pub owner: String,
    pub name: String,
    pub created_time: String,
    pub updated_time: String,
    pub display_name: String,
    pub description: String,
    pub model: String,
    pub adapter: String,
    pub organization: String,
    pub is_enabled: bool,
    pub auto_build: bool,
    pub auto_sync: bool,
    pub watcher: String,
    pub watcher_options: serde_json::Value,
    pub cache: String,
    pub cache_options: serde_json::Value,
    pub config: serde_json::Value,
}
