use crate::core::StoreLock;
use crate::service::find_record;
use rocket::State;

#[get("/<key>")]
pub fn raw_find(state: State<StoreLock>, key: String) -> String {
    match find_record(state, key) {
        Ok(res) => res.content,
        Err(e) => "Err: No such Record!".into()
    }
}