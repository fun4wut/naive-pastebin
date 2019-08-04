use std::fmt;
use std::fmt::Formatter;
use std::option::NoneError;
use crate::utils::env::*;
use std::error::Error;

#[derive(Debug)]
pub enum StoreError {
    /// record过大错误
    TooBigRecErr,
    /// 未找到record错误
    NotFoundErr(NoneError),
}

use StoreError::*;

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TooBigRecErr => write!(f, "内容太大，无法存入！"),
            NotFoundErr(_) => write!(f, "未找到该记录！")
        }
    }
}

impl Error for StoreError {
    fn description(&self) -> &str {
        match *self {
            TooBigRecErr => "内容太大，无法存入！",
            NotFoundErr(_) => "未找到该记录！"
        }
    }
}

// 将NoneError转为StoreError
impl From<NoneError> for StoreError {
    fn from(e: NoneError) -> Self {
        NotFoundErr(e)
    }
}