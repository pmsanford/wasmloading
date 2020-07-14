use wasm_bindgen::prelude::*;
use std::str::from_utf8;

const MEM_LEN: usize = 4_096;

static mut WRITE_BUF: [u8; MEM_LEN] = [0u8; MEM_LEN];

#[wasm_bindgen]
pub fn get_ptr() -> *const u8 {
    unsafe {
        WRITE_BUF.as_ptr()
    }
}

pub fn get_string() -> String {
    unsafe {
        let mut len: u16 = WRITE_BUF[0] as u16;
        len <<= 8;
        len += WRITE_BUF[1] as u16;
        if len > MEM_LEN as u16 - 2 {
            panic!("This string is too long");
        }
        from_utf8(&WRITE_BUF[0..len as usize]).unwrap().to_string()
    }
}
