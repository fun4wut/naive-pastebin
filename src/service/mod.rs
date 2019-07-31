//! 服务层
//!
//! 负责定时GC，和存取操作
mod task;
mod save;
mod find;
mod embed;
pub use save::save_record;
pub use find::find_record;
pub use task::gc_loop;
pub use embed::gen_embed;