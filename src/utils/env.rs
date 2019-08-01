//! MAX_STORE_SIZE: 100 MB
//!
//! MAX_POST_SIZE: 32 KB
//!
//! MAX_EXPIRATION: 7 days
//!
//! CLEAN_DURATION: 5000 ms
//!
//! ADDR: localhost
//!
//! PORT: 8085
//!
//! CRYPT_KEY: "fun4wut"
//!
//! DOMAIN: `ADDR:PORT`
use std::str::FromStr;
use std::env;
use super::time::*;

// 默认的服务地址
const DEFAULT_ADDR: &'static str = "localhost";

// 默认的短网址加密密钥
const DEFAULT_CRYPT_KEY: &'static str = "fun4wut";

// 解析环境变量
fn parse<T: FromStr>(key: &'static str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or(default)
}

// 懒初始化
lazy_static! {
    pub static ref MAX_STORE_SIZE: usize = { parse("PASTEBIN_MAX_STORE_SIZE", 100 * 1024 * 1024) };
    pub static ref MAX_POST_SIZE: usize = { parse("PASTEBIN_MAX_POST_SIZE", 32 * 1024) };
    pub static ref MAX_EXPIRATION: SecTime = { parse("PASTEBIN_MAX_EXPIRATION", 7 * 24 * 60 * 60) };
    pub static ref CLEAN_DURATION: u64 = { parse("PASTEBIN_CLEAN_DURATION", 5000) };
    pub static ref ADDR: String = { env::var("PASTEBIN_ADDR").unwrap_or(DEFAULT_ADDR.into()) };
    pub static ref PORT: u16 = {parse("PASTEBIN_PORT", 8085)};
    pub static ref CRYPT_KEY: String =
        { env::var("PASTEBIN_CRYPT_KEY").unwrap_or(DEFAULT_CRYPT_KEY.into()) };
    pub static ref DOMAIN: String = {env::var("PASTEBIN_DOMAIN").unwrap_or(format!("{}:{}", *ADDR, *PORT))};
}

/// 打印环境变量
pub fn info_env() {
    info!("MAX_STORE_SIZE: {} bytes", *MAX_STORE_SIZE);
    info!("MAX_EXPIRATION: {} s", *MAX_EXPIRATION);
    info!("CLEAN_DURATION: {} ms", *CLEAN_DURATION);
    info!("MAX_POST_SIZE: {} bytes", *MAX_POST_SIZE);
    info!("SERVICE_ADDRESS: {}", *DOMAIN);
}