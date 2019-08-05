#![allow(dead_code, unused_variables, unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait, box_syntax)]

mod core;
mod utils;
mod domain;
mod controller;
mod service;
mod dto;
mod server;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use crate::utils::env::info_env;
use server::rocket;

/// init, start server
fn main() {
    dotenv::dotenv().ok(); // 载入 .env 文件的变量
    env_logger::init(); // 初始化logger
    info_env();
    if let Ok(r) = rocket() {
        r.launch();
    } else { error!("服务器启动失败!"); }
}
