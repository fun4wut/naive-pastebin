//! 服务层
//!
//! 负责定时GC，和存取操作
mod embed;
mod find;
mod save;
mod task;

pub use embed::gen_embed;
pub use find::find_record;
pub use save::save_record;
pub use task::gc_loop;
