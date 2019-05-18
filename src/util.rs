//! Small utility functions.
//!
//! From [dodrio/examples/todomvc](https://github.com/fitzgen/dodrio/blob/master/examples/todomvc/src/utils.rs)

use dodrio::bumpalo::{self, Bump};
use std::fmt::Display;
use wasm_bindgen::UnwrapThrowExt;

/// Get the top-level window.
pub fn window() -> web_sys::Window {
    web_sys::window().unwrap_throw()
}

/// Get the current location hash, if any.
pub fn hash() -> Option<String> {
    window()
        .location()
        .hash()
        .ok()
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
}

/// Set the current location hash.
pub fn set_hash(hash: &str) {
    window().location().set_hash(hash).unwrap_throw();
}

/// Get the top-level document.
pub fn document() -> web_sys::Document {
    window().document().unwrap_throw()
}

/// Allocate something Display as a string and return a ref.
pub fn bump_str<'bump>(val: impl Display, bump: &'bump Bump) -> &'bump str {
    bumpalo::format!(in bump, "{}", val).into_bump_str()
}
