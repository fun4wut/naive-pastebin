use crate::core::StoreLock;
use crate::dto::{SaveReq, SaveRes};
use crate::utils::env::*;
use std::io::Error;
use crate::utils::time::{now_nano, nano_to_sec, sec_to_nano};
use crate::domain::record::Record;
use crate::domain::key::nano_to_key;

/// 传递一个Result，这里偷懒，Error直接用字符串了
pub fn save_record(store_lock: StoreLock, dto: SaveReq) -> Result<SaveRes, String> {
    if dto.expiration > *MAX_EXPIRATION {
        return Err(format!("过期时间超出最大值！最大值为：{}", *MAX_EXPIRATION));
    }
    let now = now_nano();
    let saving_time = nano_to_sec(now);
    let dead_time = now + sec_to_nano(dto.expiration);
    // 构造record
    let record = Record {
        expiration: dto.expiration,
        saving_time,
        dead_time,
        lang: dto.lang,
        title: dto.title,
        content: dto.content,
    };

    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = store_lock.write().unwrap();

    // 错误上抛
    store.save(now, record)?;

    let store_size = store.total_value_size();
    let item_count = store.item_count();
    let key = nano_to_key(now);

    info!(
        "SAVE key = {}, store_size = {}, item_count = {}",
        key, store_size, item_count
    );
    Ok(SaveRes { key })
}