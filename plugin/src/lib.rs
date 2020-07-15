use regex::Regex;
use shared::get_log_line;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_get_line() -> u8 {
    match get_log_line() {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[wasm_bindgen]
pub fn should_filter() -> u8 {
    let r = Regex::new("bb+").unwrap();
    let ll = get_log_line().unwrap();

    if r.is_match(&ll.message) {
        1
    } else {
        0
    }
}
