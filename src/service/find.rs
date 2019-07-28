use crate::core::StoreLock;
use crate::domain::key::key_to_nano;
use crate::dto::FindRes;
use std::error::Error;
use std::option::NoneError;
use rocket::State;


/// 从store中找到
pub fn find_record(store_lock: State<StoreLock>, key: String) -> Result<FindRes, NoneError> {
    let nano = key_to_nano(&key)?;
    // write store
    // assert: store_lock.write never returns Err or paincs
    let mut store = store_lock.write().unwrap();
    // access record
    let item = store.access(nano)?;

    // 构造Response
    let resp = FindRes {
        title: item.value.title.clone(),
        lang: item.value.lang.clone(),
        content: item.value.content.clone(),
        saving_time: item.value.saving_time,
        expiration: item.value.expiration,
        view_count: item.access_count,
    };

    info!("FIND key = {}", key);
    Ok(resp)
}