use js_sys::Error;
use std::fmt;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SecurityError;

/// Go back 1 page
pub fn back() {
    go(-1)
}

/// Go forwrad 1 page
pub fn forward() {
    go(1)
}

/// Reload the current page
pub fn refresh() {
    go(0)
}

/// Go back/forwrad `delta` pages (negative to go back, positive to go forward, 0 to refresh
/// current).
pub fn go(delta: i32) {
    history().go_with_delta(delta).unwrap_throw()
}

/// Push a new url onto the history stack.
///
/// This won't change the page, but will fire a `popstate` event.
///
/// # Throws
///
/// SecurityError - incorrect origin
///
pub fn push(url: &str) -> Result<(), SecurityError> {
    history()
        .push_state_with_url(&JsValue::NULL, "", Some(url))
        .map_err(|e| {
            let e = e.dyn_into::<js_sys::Error>().unwrap_throw();
            if e.name() == "SecurityError" {
                SecurityError
            } else {
                wasm_bindgen::throw_val(e.into());
            }
        })?;
    dispatch_popstate();
    Ok(())
}

/// Replace the current url with a new one.
///
/// This won't change the page, but will fire a `popstate` event.
pub fn replace(url: &str) -> Result<(), SecurityError> {
    history()
        .replace_state_with_url(&JsValue::NULL, "", Some(url))
        .map_err(|e| {
            let e = e.dyn_into::<js_sys::Error>().unwrap_throw();
            if e.name() == "SecurityError" {
                SecurityError
            } else {
                wasm_bindgen::throw_val(e.into());
            }
        })?;
    dispatch_popstate();
    Ok(())
}

// utility

fn history() -> web_sys::History {
    window().history().unwrap_throw()
}

fn window() -> web_sys::Window {
    web_sys::window().unwrap_throw()
}

fn dispatch_popstate() {
    let evt = web_sys::Event::new("popstate").unwrap_throw();
    window().dispatch_event(&evt).unwrap_throw();
}
