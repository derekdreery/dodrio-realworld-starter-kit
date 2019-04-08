#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn push_state() {
    assert!(wasm_history::push("https://github.com/ney").is_err());
    assert!(wasm_history::push("new_page.html").is_ok());
    assert!(wasm_history::push("../new_page.html").is_ok());
}
