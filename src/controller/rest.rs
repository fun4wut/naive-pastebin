use crate::core::StoreLock;
use rocket::State;
use rocket_contrib::json::Json;
use crate::dto::{SaveReq, ResDTO, SaveRes, FindRes};
use crate::service::{find_record, save_record};

type Res<T> = Json<ResDTO<T>>;

#[post("/save", format = "json", data = "<req>")]
pub fn save(state: State<StoreLock>, req: Json<SaveReq>) -> Res<SaveRes> {
    let dto = match save_record(state, req.0) {
        Ok(res) => ResDTO::success(res),
        Err(e) => ResDTO::error().with_msg(e)
    };
    Json(dto)
}

#[get("/find/<key>")]
pub fn find(key: String, state: State<StoreLock>) -> Res<FindRes> {
    let dto = match find_record(state, key) {
        Ok(res) => ResDTO::success(res),
        Err(_) => ResDTO::error().with_msg("未找到内容！".into())
    };
    Json(dto)
}