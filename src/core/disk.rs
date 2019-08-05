//! 对于冷数据，落盘存储
//!
//! 将 Key 的前2位作为一级目录，3-4位作为二级目录

use std::hash::Hash;
use crate::core::{LruValueSize, WithDeadTime};
use serde::{Serialize, Deserialize};
use sha1::{Sha1, Digest};
use std::fs::{File, create_dir_all, remove_file};
use std::path::Path;
use bincode::{serialize_into, deserialize_from};
use std::marker::PhantomData;
use std::error::Error;
use serde::de::DeserializeOwned;
use crate::utils::error::StoreError;
use std::option::NoneError;
use crate::utils::time::{ToArray, now_nano};

/// helper function
#[inline]
fn enc_to_str<T: ToArray>(data: T) -> String {
    Sha1::from(&data.to_array()).hexdigest()
}

/// 将item存入硬盘
pub struct DiskStore<K, V>
    where
        K: ToArray,
        V: Serialize + DeserializeOwned + WithDeadTime // 使用 DesOwned 而不是 Des，绕开生命周期限制
{
    /// 无用
    pd: PhantomData<K>,
    /// 无用
    pd2: PhantomData<V>,
}

impl<K, V> DiskStore<K, V>
    where
        K: ToArray,
        V: Serialize + DeserializeOwned + WithDeadTime
{
    pub fn new() -> Self {
        Self {
            pd: PhantomData,
            pd2: PhantomData,
        }
    }

    /// 将item写入硬盘
    pub fn save(&self, stamp: K, item: V) -> Result<(), StoreError> {
        let key = enc_to_str(stamp);
        let path = format!("./data/{}/{}/", &key[..2], &key[2..4]);
        create_dir_all(&path)?;
        let writer = File::create(path + &key)?;
        serialize_into(writer, &item)?;
        Ok(())
    }


    /// 将item从硬盘中导出
    pub fn find(&self, stamp: K) -> Result<V, StoreError> {
        let key = enc_to_str(stamp);
        let path = format!("./data/{}/{}/{}", &key[..2], &key[2..4], &key);
        // 有该item
        if Path::new(&path).exists() {
            let reader = File::open(&path)?;
            let res: V = deserialize_from(reader)?;
            match &res.dead_time() {
                Some(time) if *time < now_nano() => {
                    // 如果硬盘上的item已经过期，将其删除，并返回错误
                    remove_file(&path)?;
                    Err(StoreError::NotFoundErr)
                },
                _ => Ok(res)
            }
        } else {
            Err(StoreError::NotFoundErr)
        }
    }
}