#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn push_state() {
    wasm_history::push("new_page.html").unwrap_throw();
    wasm_history::push("../new_page.html").unwrap_throw();
}
