use crate::core::StoreLock;
use crate::service::find_record;
use rocket::State;
use std::error::Error;
use std::path::PathBuf;

#[get("/<key..>")]
/// 因为不知道怎么使用通配符，所以只能全接手过来再做处理了。。
pub fn raw_find(state: State<StoreLock>, key: PathBuf) -> String {
    match find_record(state, key.as_path().iter().next().unwrap().to_str().unwrap().into()) {
        Ok(res) => res.content,
        Err(e) => e.description().into()
    }
}