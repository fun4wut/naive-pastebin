use crate::utils::time::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// 获取Value的大小
pub trait LruValueSize {
    fn lru_value_size(&self) -> usize;
}

/// 过期时间
pub trait WithDeadTime {
    /// 返回值为 `None` 即为永不过期
    fn dead_time(&self) -> Option<NanoTime>;
}

/// 对Value实现的一个包装
/// V要实现 `LruValueSize` 和 序列化/反序列化
pub struct StoreItem<V>
    where
        V: LruValueSize + DeserializeOwned + Serialize,
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
        V: LruValueSize + DeserializeOwned + Serialize,
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

#[cfg(test)]
mod tests {
    use crate::core::store_item::StoreItem;
    use crate::core::{LruValueSize, WithDeadTime};
    use crate::utils::time::NanoTime;

    struct Foo {
        size: usize,
        time: Option<NanoTime>,
    }

    impl LruValueSize for Foo {
        fn lru_value_size(&self) -> usize {
            self.size
        }
    }

    impl WithDeadTime for Foo {
        fn dead_time(&self) -> Option<NanoTime> {
            self.time
        }
    }

    #[test]
    fn test_store_item() {
        let item = StoreItem::new(Foo {
            time: Some(2000),
            size: 500,
        });
        assert_eq!(item.value.dead_time().unwrap(), 2000);
        assert_eq!(item.value.lru_value_size(), 500);
    }
}
