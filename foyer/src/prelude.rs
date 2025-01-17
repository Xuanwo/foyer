//  Copyright 2024 Foyer Project Authors
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use crate::memory;
use crate::storage;

use ahash::RandomState;
pub use memory::{CacheContext, EvictionConfig, FifoConfig, LfuConfig, LruConfig, S3FifoConfig};
pub use storage::{
    AdmissionContext, AdmissionPolicy, Compression, ExistReinsertionPolicy, FsDeviceConfig, FsDeviceConfigBuilder,
    RatedTicketAdmissionPolicy, RatedTicketReinsertionPolicy, ReinsertionContext, ReinsertionPolicy,
    RuntimeConfigBuilder, Storage, StorageExt,
};

pub type Cache<K, V, S = RandomState> = memory::Cache<K, V, memory::DefaultCacheEventListener<K, V>, S>;
pub type CacheBuilder<K, V, S> = memory::CacheBuilder<K, V, memory::DefaultCacheEventListener<K, V>, S>;

pub use crate::hybrid::{
    HybridCache, HybridCacheBuilder, HybridCacheBuilderPhaseMemory, HybridCacheBuilderPhaseStorage,
};
