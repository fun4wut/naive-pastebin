use crate::core::StoreLock;
use crate::dto::FindRes;
use crate::service::{find_record, gen_embed};
use crate::utils::env::*;
use rocket::http::{ContentType, Status};
use rocket::{Response, State};
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::io::Cursor;

#[get("/embed/<key>")]
pub fn show_embed(state: State<StoreLock>, key: String) -> Response {
    let mut res = Response::new();
    res.set_header(ContentType::JavaScript);
    match gen_embed(state, key) {
        Ok(js) => res.set_sized_body(Cursor::new(js)),
        Err(_) => res.set_status(Status::NotFound),
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
        Ok(res) => Some(Template::render(
            "show",
            &Show {
                rec: &res,
                key: clone,
                domain: &*DOMAIN,
            },
        )),
        Err(_) => None,
    }
}

#[get("/embed_iframe/<key>")]
pub fn show_embed_iframe(state: State<StoreLock>, key: String) -> Option<Template> {
    let clone = key.clone();
    match find_record(state, key) {
        Ok(res) => Some(Template::render(
            "embed",
            &Show {
                rec: &res,
                key: clone,
                domain: &*DOMAIN,
            },
        )),
        Err(_) => None,
    }
}
