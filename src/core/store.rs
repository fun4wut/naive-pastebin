use std::hash::Hash;
use crate::core::store_item::{LruValueSize, WithDeadTime, StoreItem};
use linked_hash_map::LinkedHashMap;
use std::collections::BTreeMap;
use crate::utils::time::{NanoTime, sec_to_nano};
use std::collections::btree_map::Entry;

/// 存储结构
/// Value需要实现两个Trait
pub struct Store<K, V>
    where
        K: Copy + Hash + Eq,
        V: LruValueSize + WithDeadTime
{
    /// 记录（K，V）
    map: LinkedHashMap<K, StoreItem<V>>,
    /// 过期检测与删除
    queue: BTreeMap<NanoTime, K>,
    /// 当前的容量
    total_value_size: usize,
    /// 最大容量
    max_value_size: usize,
}

impl<K, V> Store<K, V>
    where
        K: Copy + Hash + Eq,
        V: LruValueSize + WithDeadTime
{
    /// 构造一个Store实例
    pub fn new(max_value_size: usize) -> Self {
        Self {
            map: LinkedHashMap::new(),
            queue: BTreeMap::new(),
            total_value_size: 0,
            max_value_size,
        }
    }

    /// 存储 `K,V` 键值对
    pub fn save(&mut self, key: K, value: V) -> Result<(), String> {
        let item = StoreItem::new(value);

        // 确保Store的最大容量能容纳这个item
        if item.size >= self.max_value_size {
            return Err("内容太大，无法存入！".into());
        }

        // LRU淘汰掉老item，直到有空间来存放item
        while self.max_value_size - self.total_value_size < item.size {
            if let Some((_, it)) = self.map.pop_front() {
                self.total_value_size -= it.size;
                self.queue.remove(&it.value.dead_time());
            }
        }
        self.total_value_size += item.size;
        let mut dead_time = item.value.dead_time();
        // 处理过期时间碰撞
        loop {
            let entry = self.queue.entry(dead_time);// 寻找具有该过期时间的entry
            if let Entry::Vacant(_) = entry { // 如果没找到
                entry.or_insert(key); // 插入该entry
                break;
            }
            dead_time += sec_to_nano(1); // 如果有碰撞，过期时间+1s
            info!("dead_time collision: {}", dead_time);
        }
        // 存放至LRU
        self.map.insert(key, item);
        Ok(())
    }

    /// 根据key来访问
    pub fn access(&mut self, key: K) -> Option<&StoreItem<V>> {
        let item = self.map.get_refresh(&key)?; // 获取item，更新LRU
        item.access_count += 1;
        Some(item)
    }

    /// 清洗过期的item，返回被清洗的item数量
    pub fn clean(&mut self, now: NanoTime) -> usize {
        // 分割过期列表, 过期时间大于当前时间的不会被清除
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
}