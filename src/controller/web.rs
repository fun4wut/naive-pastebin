use rocket::{State, Response};
use crate::core::StoreLock;
use rocket::http::{ContentType, Status};
use crate::service::gen_embed;
use std::io::Cursor;

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