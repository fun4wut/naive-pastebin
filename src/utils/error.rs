use crate::utils::env::*;
use bincode::ErrorKind;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::option::NoneError;
#[derive(Debug)]
pub enum StoreError {
    /// record过大错误
    TooBigRecErr,
    /// 未找到record错误
    NotFoundErr,
    /// IO错误
    IOErr(std::io::Error),
    /// bincode序列化/反序列化错误
    BinCodeErr(Box<ErrorKind>),
    /// 空错误
    NoneErr,
}

use StoreError::*;

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TooBigRecErr => write!(f, "内容太大，无法存入！"),
            NotFoundErr => write!(f, "未找到该记录！"),
            IOErr(e) => write!(f, "{}", e.description()),
            BinCodeErr(e) => write!(f, "{}", e.description()),
            NoneErr => write!(f, "空值！"),
        }
    }
}

impl Error for StoreError {
    fn description(&self) -> &str {
        match self {
            TooBigRecErr => "内容太大，无法存入！",
            NotFoundErr => "未找到该记录！",
            IOErr(e) => e.description(),
            BinCodeErr(e) => e.description(),
            NoneErr => "空值！",
        }
    }
}

impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        IOErr(e)
    }
}

impl From<Box<ErrorKind>> for StoreError {
    fn from(e: Box<ErrorKind>) -> Self {
        BinCodeErr(e)
    }
}

impl From<NoneError> for StoreError {
    fn from(_: NoneError) -> Self {
        NoneErr
    }
}
