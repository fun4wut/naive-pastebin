//! 对于冷数据，落盘存储
//!
//! 将 Key 的前2位作为一级目录，3-4位作为二级目录

use crate::core::{LruValueSize, WithDeadTime};
use crate::utils::error::StoreError;
use crate::utils::time::{now_nano, ToArray};
use bincode::{deserialize_from, serialize_into};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::error::Error;
use std::fs::{create_dir_all, remove_file, File};
use std::hash::Hash;
use std::marker::PhantomData;
use std::option::NoneError;
use std::path::Path;
use std::io::{BufWriter, Write};
use tokio::fs;
use tokio::prelude::Future;

/// helper function
#[inline]
fn enc_to_str<T: ToArray>(data: T) -> String {
    Sha1::from(&data.to_array()).hexdigest()
}

/// 将item存入硬盘
pub struct DiskStore<K, V>
    where
        K: ToArray,
        V: Serialize + DeserializeOwned + WithDeadTime + Sync + Send + 'static, // 使用 DesOwned 而不是 Des，绕开生命周期限制
{
    /// 无用
    pd: PhantomData<K>,
    /// 无用
    pd2: PhantomData<V>,
}

impl<K, V> DiskStore<K, V>
    where
        K: ToArray,
        V: Serialize + DeserializeOwned + WithDeadTime + Sync + Send + 'static,
{
    /// 异步写入磁盘
    /// 比同步方式，提升4倍速度
    pub fn save_async(stamp: K, item: V) -> Result<(), StoreError> {
        let key = enc_to_str(stamp);
        let path = format!("./data/{}/{}/", &key[..2], &key[2..4]);
        let fut = fs::create_dir_all(path.clone())
            .and_then(move |_| fs::File::create(format!("{}{}", path, key)))
            .map(move |f| serialize_into(f, &item).unwrap())
            .map_err(|e| eprintln!("rua"));
        Ok(tokio::run(fut))
    }
    /// 将item写入硬盘
    pub fn save(stamp: K, item: V) -> Result<(), StoreError> {
        let key = enc_to_str(stamp);
        let path = format!("./data/{}/{}/", &key[..2], &key[2..4]);
        create_dir_all(&path)?;
        let writer = BufWriter::new(File::create(path + &key)?);
        serialize_into(writer, &item)?;
        Ok(())
    }

    /// 将item从硬盘中导出
    pub fn find(stamp: K) -> Result<V, StoreError> {
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
                }
                _ => Ok(res),
            }
        } else {
            Err(StoreError::NotFoundErr)
        }
    }
}
