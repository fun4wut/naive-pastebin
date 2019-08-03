//! 数据传输时的object

use crate::utils::time::SecTime;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// 保存文件的JSON请求
#[derive(Serialize, Deserialize)]
pub struct SaveReq {
    /// 内容
    pub content: String,
}


/// 返回的JSON，包装的最外层对象
#[derive(Serialize)]
pub struct ResDTO<T: Serialize> {
    pub code: isize,
    pub msg: String,
    pub data: Option<T>,
}


impl<T> ResDTO<T> where T: Serialize {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: "success".into(),
            data: Some(data),
        }
    }
    pub fn error() -> Self {
        Self {
            code: -1,
            msg: "error".into(),
            data: None,
        }
    }
    pub fn with_code(mut self, code: isize) -> Self {
        self.code = code;
        self
    }
    pub fn with_msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }
}

#[derive(Serialize)]
pub struct SaveRes {
    pub key: Arc<String>
}

#[derive(Serialize)]
pub struct FindRes {
    pub title: Arc<String>,
    pub content: Arc<String>,
    pub saving_time: SecTime,
    pub expiration: SecTime,
    pub view_count: u64,
}