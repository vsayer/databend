//  Copyright 2021 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use async_trait::async_trait;
use common_kv_api_vo::GetKVActionResult;
use common_kv_api_vo::MGetKVActionResult;
use common_kv_api_vo::PrefixListReply;
use common_kv_api_vo::UpsertKVActionResult;
use common_metatypes::KVMeta;
use common_metatypes::MatchSeq;

#[async_trait]
pub trait KVApi: Send + Sync {
    async fn upsert_kv(
        &self,
        key: &str,
        seq: MatchSeq,
        value: Option<Vec<u8>>,
        value_meta: Option<KVMeta>,
    ) -> common_exception::Result<UpsertKVActionResult>;

    async fn update_kv_meta(
        &self,
        key: &str,
        seq: MatchSeq,
        value_meta: Option<KVMeta>,
    ) -> common_exception::Result<UpsertKVActionResult>;

    async fn get_kv(&self, key: &str) -> common_exception::Result<GetKVActionResult>;

    // mockall complains about AsRef... so we use String here
    async fn mget_kv(&self, key: &[String]) -> common_exception::Result<MGetKVActionResult>;

    async fn prefix_list_kv(&self, prefix: &str) -> common_exception::Result<PrefixListReply>;
}
