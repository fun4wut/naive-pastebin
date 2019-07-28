#![allow(dead_code, unused_variables)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]

mod core;
mod utils;
mod domain;
mod controller;
mod service;
mod dto;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use crate::utils::env::info_env;

fn main() {
    dotenv::dotenv().ok(); // 载入 .env 文件的变量
    env_logger::init(); // 初始化logger
    info_env();
}
