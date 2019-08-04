use crate::core::StoreLock;
use rocket::State;
use rocket_contrib::json::Json;
use crate::dto::{SaveReq, ResDTO, SaveRes, FindRes};
use crate::service::{find_record, save_record};
use crate::utils::time::SecTime;
use crate::utils::env::*;
type Res<T> = Json<ResDTO<T>>;

#[post("/save?<exp>&<title>", format = "json", data = "<req>")]
pub fn save(state: State<StoreLock>, req: Json<SaveReq>, exp: Option<SecTime>, title: String) -> Res<SaveRes> {
    let dto = match save_record(state, req.0, title, exp) {
        Ok(res) => ResDTO::success(res),
        Err(e) => ResDTO::error().with_msg(e.to_string())
    };
    Json(dto)
}

#[get("/find/<key>")]
pub fn find(key: String, state: State<StoreLock>) -> Res<FindRes> {
    let dto = match find_record(state, key) {
        Ok(res) => ResDTO::success(res),
        Err(e) => ResDTO::error().with_msg(e.to_string())
    };
    Json(dto)
}
