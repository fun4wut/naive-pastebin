use crate::core::StoreLock;
use crate::domain::key::nano_to_key;
use crate::domain::record::Record;
use crate::dto::{SaveReq, SaveRes};
use crate::utils::env::*;
use crate::utils::error::StoreError;
use crate::utils::time::{nano_to_sec, now_nano, sec_to_nano, NanoTime, SecTime};
use rand::random;
use rocket::State;
use std::sync::Arc;
/// 存储record
pub fn save_record(
    store_lock: State<StoreLock>,
    dto: SaveReq,
    title: String,
    exp: Option<SecTime>,
) -> Result<SaveRes, StoreError> {
    let mut now = now_nano();
    let saving_time = nano_to_sec(now);

    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = store_lock.write().unwrap();

    // 防止冲突，加上随机数
    while store.contains(now) {
        now += random::<u8>() as NanoTime;
    }

    let dead_time = exp.map(|e| now + sec_to_nano(e));
    // 构造record
    let record = Record {
        expiration: exp,
        saving_time,
        dead_time,
        title: Arc::new(title),
        content: Arc::new(dto.content),
    };

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
