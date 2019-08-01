use rocket::{State, Response};
use crate::core::StoreLock;
use rocket::http::{ContentType, Status};
use crate::service::gen_embed;
use std::io::Cursor;
use rocket_contrib::templates::{Template};
use crate::dto::SaveRes;
use crate::utils::env::*;
use serde::Serialize;
use rocket::request::Form;
use crate::utils::time::SecTime;

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
struct Index<'a> {
    domain: &'a str
}

#[get("/")]
pub fn index() -> Template {
    let context = Index { domain: &*DOMAIN };
    Template::render("index", &context)
}

#[derive(FromForm)]
struct SaveForm {
    title: String,
    exp: SecTime,
    content: String,
}
