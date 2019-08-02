use rocket::{State, Response};
use crate::core::StoreLock;
use rocket::http::{ContentType, Status};
use crate::service::{gen_embed, find_record};
use std::io::Cursor;
use rocket_contrib::templates::{Template};
use crate::dto::{FindRes};
use crate::utils::env::*;
use serde::Serialize;

#[get("/embed/<key>")]
pub fn show_embed(state: State<StoreLock>, key: String) -> Response {
    let mut res = Response::new();
    res.set_header(ContentType::JavaScript);
    match gen_embed(state, key) {
        Ok(js) => res.set_sized_body(Cursor::new(js)),
        Err(_) => res.set_status(Status::NotFound)
    };
    res
}

#[derive(Serialize)]
struct Index {}

#[get("/")]
pub fn index() -> Template {
    let context = Index {};
    Template::render("index", &context)
}

#[derive(Serialize)]
struct Show<'a> {
    rec: &'a FindRes,
    key: String,
    domain: &'a str,
}

#[get("/show/<key>")]
pub fn show_record(state: State<StoreLock>, key: String) -> Option<Template> {
    let clone = key.clone();
    match find_record(state, key) {
        Ok(res) => {
            Some(Template::render("show", &Show {
                rec: &res,
                key: clone,
                domain: &*DOMAIN,
            }))
        },
        Err(_) => None
    }
}