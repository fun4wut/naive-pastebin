use crate::controller::raw::*;
use crate::controller::rest::*;
use crate::controller::web::*;
use crate::core::create_store_lock;
use crate::service::gc_loop;
use crate::utils::env::*;
use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::error::Error;

pub fn rocket() -> Result<rocket::Rocket, Box<dyn Error>> {
    let store_lock = create_store_lock(*MAX_STORE_SIZE);
    gc_loop(store_lock.clone());
    let cfg = Config::build(Environment::Staging)
        .address(&*ADDR)
        .port(*PORT)
        .finalize()?;
    Ok(rocket::custom(cfg)
        .manage(store_lock)
        .attach(Template::fairing())
        .mount(
            "/",
            routes![show_embed, index, show_record, show_embed_iframe],
        )
        .mount("/api", routes![find, save])
        .mount("/raw", routes![raw_find])
        // serve static files
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        ))
}
