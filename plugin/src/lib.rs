use regex::Regex;
use shared::get_string;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn should_filter() -> u8 {
    let r = Regex::new("bb+").unwrap();
    let s = get_string();

    if r.is_match(&s) {
        1
    } else {
        0
    }
}
