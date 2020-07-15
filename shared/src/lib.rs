use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const MEM_LEN: usize = 4_096;

static mut WRITE_BUF: [u8; MEM_LEN] = [0u8; MEM_LEN];

#[derive(Serialize, Deserialize)]
pub struct LogLine {
    pub message: String,
    // more fields
}

impl LogLine {
    pub fn new(message: String) -> Self {
        LogLine { message }
    }
}

#[wasm_bindgen]
pub fn get_ptr() -> *const u8 {
    unsafe { WRITE_BUF.as_ptr() }
}

pub fn get_log_line() -> Result<LogLine> {
    unsafe { Ok(bincode::deserialize(&WRITE_BUF[..])?) }
}
