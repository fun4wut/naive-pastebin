use crate::core::StoreLock;
use crate::dto::{SaveReq, SaveRes};
use crate::utils::env::*;
use crate::utils::time::{now_nano, nano_to_sec, sec_to_nano, SecTime};
use crate::domain::record::Record;
use crate::domain::key::nano_to_key;
use rocket::State;
use crate::utils::error::StoreError;
use std::sync::Arc;

/// 存储record
pub fn save_record(store_lock: State<StoreLock>, dto: SaveReq, title: String, exp: SecTime)
                   -> Result<SaveRes, StoreError> {
    if exp > *MAX_EXPIRATION {
        // 如果过期时间超过最大值，抛出错误
        return Err(StoreError::ExpOverflowErr);
    }
    let now = now_nano();
    let saving_time = nano_to_sec(now);
    let dead_time = now + sec_to_nano(exp);
    // 构造record
    let record = Record {
        expiration: exp,
        saving_time,
        dead_time,
        title: Arc::new(title),
        content: Arc::new(dto.content),
    };

    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = store_lock.write().unwrap();

    // 错误上抛
    store.save(now, record)?;

    let store_size = store.total_value_size();
    let item_count = store.item_count();
    let key = Arc::new(nano_to_key(now));

    info!(
        "SAVE key = {}, store_size = {}, item_count = {}",
        key, store_size, item_count
    );
    Ok(SaveRes { key })
}