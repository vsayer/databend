// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;

use crate::error::Result;

/// `Delete` will invoke the `delete` operation.
///
/// ## Behavior
///
/// - `Delete` is an idempotent operation, it's safe to call `Delete` on the same path multiple times.
/// - `Delete` will return `Ok(())` if the path is deleted successfully or not exist.
#[async_trait]
pub trait Delete<S: Send + Sync>: Send + Sync {
    async fn delete(&self, path: &str) -> Result<()> {
        let _ = path;
        unimplemented!()
    }
}
