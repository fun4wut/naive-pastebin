use crate::core::create_store_lock;
use crate::utils::env::*;
use crate::controller::rest::*;
use crate::service::gc_loop;
use rocket::config::{Config, Environment};
use std::error::Error;
use crate::controller::raw::*;

pub fn rocket() -> Result<rocket::Rocket, Box<dyn Error>> {
    let store_lock = create_store_lock(*MAX_STORE_SIZE);
    gc_loop(store_lock.clone());
    let cfg = Config::build(Environment::Staging)
        .address(&*ADDR)
        .port(*PORT)
        .finalize()?;
    Ok(rocket::custom(cfg)
        .manage(store_lock)
        .mount("/api", routes![find,save])
        .mount("/raw", routes![raw_find]))
}