//! Value实体
//!
//! 记录标题，语言，内容，时间等数据
use crate::core::{LruValueSize, WithDeadTime};
use crate::utils::time::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Record {
    pub title: Arc<String>,
    pub content: Arc<String>,
    pub saving_time: SecTime,
    pub expiration: SecTime,
    pub dead_time: NanoTime,
}

impl Record {
    /// escape the code to html style
    pub fn escape(&self) -> String {
        self.content.replace("<", "&lt;")
            .replace(">", "&gt;")
    }
}

impl LruValueSize for Record {
    /// the size of lru_value
    fn lru_value_size(&self) -> usize {
        // size on stack + size on heap
        std::mem::size_of::<Self>()
            + self.title.as_bytes().len()
            + self.content.as_bytes().len()
    }
}

impl WithDeadTime for Record {
    /// the dead time of lru_value
    fn dead_time(&self) -> NanoTime {
        self.dead_time
    }
}
