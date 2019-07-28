use crate::utils::time::*;

/// 获取Value的大小
pub trait LruValueSize {
    fn lru_value_size(&self) -> usize;
}

/// 过期时间
pub trait WithDeadTime {
    fn dead_time(&self) -> NanoTime;
}

/// 对Value实现的一个包装
/// V要实现 `LruValueSize`
pub struct StoreItem<V>
    where
        V: LruValueSize
{
    /// Value的值
    pub value: V,
    /// Value的被访问次数
    pub access_count: u64,
    /// Value的大小
    pub size: usize,
}

impl<V> StoreItem<V>
    where
        V: LruValueSize
{
    pub fn new(value: V) -> Self {
        let size = value.lru_value_size();
        Self {
            value,
            access_count: 0,
            size,
        }
    }
}



