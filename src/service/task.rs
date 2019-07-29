use crate::core::{Store, StoreLock};
use crate::utils::time::{NanoTime, now_nano};
use std::thread;
use std::time::Duration;
use crate::utils::env::*;

/// 垃圾回收
fn gc(store: &mut Store, now: NanoTime) {
    let before_size = store.total_value_size();
    let before_count = store.item_count();
    let removed_count = {
        let cnt = store.clean(now);
        store.shrink();
        cnt
    };
    let stw_time = now_nano() - now; // 垃圾回收所花费的时间
    let after_size = store.total_value_size();
    let after_count = store.item_count();

    info!(
        "CLEAN stw: {} ns, removed: {}, store_size: {} -> {}, item_count: {} -> {}",
        stw_time, removed_count, before_size, after_size, before_count, after_count
    );
}

// 开新线程，不断进行GC
pub fn gc_loop(store_lock: StoreLock) {
    // 强制获得store_lock 的所有权
    thread::spawn(move || loop {
        // write store
        // assert: store_lock.write never returns Err or paincs
        let mut store = store_lock.write().unwrap();
        let now = now_nano();
        if store.needs_clean(now) {
            gc(&mut *store, now);
        }

        // 释放写者锁
        drop(store);

        // 线程睡眠
        thread::sleep(Duration::from_millis(*CLEAN_DURATION));
    });
}
