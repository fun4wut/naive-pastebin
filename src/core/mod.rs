//! 核心数据结构模块
mod disk;
mod store;
mod store_item;

pub use store_item::{LruValueSize, WithDeadTime}; // 只导出两个trait

use crate::domain::record::Record;
use crate::utils::time::NanoTime;
use std::sync::{Arc, RwLock};

// 包装Store，并导出
pub type Store = store::Store<NanoTime, Record>;
// 使用读写锁，保证线程安全
pub type StoreLock = Arc<RwLock<Store>>;

pub fn create_store_lock(max_size: usize) -> StoreLock {
    Arc::new(RwLock::new(Store::new(max_size)))
}
