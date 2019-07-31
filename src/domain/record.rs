//! Value实体
//!
//! 记录标题，语言，内容，时间等数据
use crate::core::{LruValueSize, WithDeadTime};
use crate::utils::time::*;

#[derive(Debug)]
pub struct Record {
    pub title: String,
    pub content: String,
    pub saving_time: SecTime,
    pub expiration: SecTime,
    pub dead_time: NanoTime,
}

impl LruValueSize for Record {
    fn lru_value_size(&self) -> usize {
        // 自身的栈大小+堆上分配内存的大小
        std::mem::size_of::<Self>()
            + self.title.as_bytes().len()
            + self.content.as_bytes().len()
    }
}

impl WithDeadTime for Record {
    fn dead_time(&self) -> NanoTime {
        self.dead_time
    }
}
