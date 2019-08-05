use crate::core::disk::DiskStore;
use crate::core::store_item::{LruValueSize, StoreItem, WithDeadTime};
use crate::utils::error::StoreError;
use crate::utils::time::{sec_to_nano, NanoTime, ToArray};
use linked_hash_map::LinkedHashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::hash::Hash;

/// 存储结构
///
/// Value需要实现两个Trait
pub struct Store<K, V>
    where
        K: Copy + Hash + Eq + ToArray,
        V: LruValueSize + WithDeadTime + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// 记录（K，V）
    map: LinkedHashMap<K, StoreItem<V>>,
    /// 过期检测与删除
    queue: BTreeMap<NanoTime, K>,

    /// 硬盘管理
    disk: DiskStore<K, V>,
    /// 当前的容量
    total_value_size: usize,
    /// 最大容量
    max_value_size: usize,
}

impl<K, V> Store<K, V>
    where
        K: Copy + Hash + Eq + ToArray,
        V: LruValueSize + WithDeadTime + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// 构造一个Store实例
    pub fn new(max_value_size: usize) -> Self {
        Self {
            map: LinkedHashMap::new(),
            queue: BTreeMap::new(),
            disk: DiskStore::new(),
            total_value_size: 0,
            max_value_size,
        }
    }

    /// 存储 `K,V` 键值对
    pub fn save(&mut self, key: K, value: V) -> Result<(), StoreError> {
        let item = StoreItem::new(value);

        // 确保Store的最大容量能容纳这个item
        if item.size >= self.max_value_size {
            return Err(StoreError::TooBigRecErr);
        }

        // LRU淘汰掉老item，直到有空间来存放item
        while self.max_value_size - self.total_value_size < item.size {
            if let Some((stamp, it)) = self.map.pop_front() {
                self.total_value_size -= it.size;

                // 如果过期时间有限，从queue中删去
                if let Some(tmp_dead_time) = it.value.dead_time() {
                    self.queue.remove(&tmp_dead_time);
                }

                self.disk.save_async(stamp, it.value)?;
            }
        }
        self.total_value_size += item.size;
        let dead_time = item.value.dead_time();

        // 处理过期时间碰撞
        // 其实过期时间碰撞概率非常低。。如果真碰撞了，对queue的remove操作就废了
        //        loop {
        //            let entry = self.queue.entry(dead_time);// 寻找具有该过期时间的entry
        //            if let Entry::Vacant(_) = entry { // 如果没找到
        //                entry.or_insert(key); // 插入该entry
        //                break;
        //            }
        //            dead_time += rand::random::<u8>() as NanoTime; // 如果有碰撞，过期时间加上一个随机数
        //            info!("dead_time collision: {}", dead_time);
        //        }

        if let Some(dt) = dead_time {
            self.queue.insert(dt, key);
        }

        // 存放至LRU
        self.map.insert(key, item);
        Ok(())
    }

    /// 根据key来访问
    ///
    /// 刷新LRU
    pub fn access(&mut self, key: K) -> Result<&StoreItem<V>, StoreError> {
        //        let item = self.map.get_refresh(&key)?; // 获取item，更新LRU
        //        item.access_count += 1;
        //        Some(item)
        //        self.map.get_refresh(&key)
        //            .ok_or(StoreError::NotFoundErr)
        //            .map(|item| {
        //                item.access_count += 1;
        //                item
        //            })

        if let Some(item) = self.map.get_refresh(&key) {
            item.access_count += 1;
            Ok(item)
        } else {
            let rec = self.disk.find(key)?;
            self.save(key, rec)?;
            Ok(self.map.get(&key)?)
        }
    }

    /// 清洗过期的item，返回被清洗的item数量
    pub fn clean(&mut self, now: NanoTime) -> usize {
        // 分割过期列表, 过期时间大于等于当前时间的不会被清除
        let right = self.queue.split_off(&now);
        let count = self.queue.len();
        let mut delta = 0;
        // 一个个删除过期的item，并返还空间
        for (_, key) in &self.queue {
            if let Some(it) = self.map.remove(&key) {
                delta += it.size;
            }
        }
        self.queue = right;
        self.total_value_size -= delta;
        count
    }

    #[inline]
    /// 判断是否需要进行过期清洗
    pub fn needs_clean(&self, now: NanoTime) -> bool {
        self.queue
            .iter()
            .next() // 只需要看最老的即可
            .map(|(&dead_time, _)| dead_time <= now)
            .unwrap_or(false)
    }

    #[inline]
    pub fn total_value_size(&self) -> usize {
        self.total_value_size
    }

    #[inline]
    pub fn item_count(&self) -> usize {
        self.map.len()
    }

    #[inline]
    /// 把LRU收缩到合适的大小
    pub fn shrink(&mut self) {
        self.map.shrink_to_fit()
    }

    #[inline]
    pub fn contains(&self, k: K) -> bool {
        self.map.contains_key(&k)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::core::{LruValueSize, WithDeadTime};
//    use crate::utils::time::now_nano;
//
//    #[derive(PartialEq, Clone, Debug)]
//    struct Bar {
//        t: Option<NanoTime>,
//    }
//
//    impl LruValueSize for Bar {
//        fn lru_value_size(&self) -> usize {
//            50
//        }
//    }
//
//    impl WithDeadTime for Bar {
//        fn dead_time(&self) -> Option<NanoTime> {
//            self.t
//        }
//    }
//
//    #[test]
//    fn test_save() {
//        let mut store: Store<usize, Bar> = Store::new(2000);
//        for i in 0..50 {
//            store
//                .save(
//                    i,
//                    Bar {
//                        t: Some(i as NanoTime),
//                    },
//                )
//                .expect("插入失败");
//        }
//        assert_eq!(store.map.len(), 40);
//        assert_eq!(store.queue.len(), 40);
//        assert_eq!(store.total_value_size(), 2000);
//    }
//
//    #[test]
//    fn test_access() {
//        let mut store: Store<usize, Bar> = Store::new(2000);
//        let v = Bar {
//            t: Some(20 as NanoTime),
//        };
//        let u = v.clone();
//        store.save(20, v).unwrap();
//        assert_eq!(store.access(20).unwrap().value, u);
//        assert!(store.access(30).is_none());
//    }
//
//    #[test]
//    fn test_clean() {
//        let mut store: Store<NanoTime, Bar> = Store::new(2000);
//        let now = now_nano();
//        for i in 0..60 {
//            store.save(i, Bar { t: Some(now + i) });
//        }
//        assert!(store.needs_clean(now + 62));
//        let num = store.clean(now + 30);
//        // 总容量只有40，所以前20个被LRU淘汰。
//        assert_eq!(num, 10);
//    }
//}
