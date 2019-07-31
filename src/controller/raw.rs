use crate::core::StoreLock;
use crate::service::find_record;
use rocket::State;
use std::error::Error;

#[get("/<key>/<_p>")]
/// p其实是不要的。。但我不知道怎么处理
pub fn raw_find(state: State<StoreLock>, key: String, _p: String) -> String {
    match find_record(state, key) {
        Ok(res) => res.content,
        Err(e) => e.description().into()
    }
}