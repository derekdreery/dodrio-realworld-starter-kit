use std::fmt;
use url::Url;
use wasm_bindgen::prelude::*;

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
pub fn push(url: &str) -> Result<(), PushUrlError> {
    check_url(url)?;
    history()
        .push_state_with_url(&JsValue::NULL, "", Some(url))
        .unwrap_throw();
    Ok(())
}

/// Replace the current url with a new one.
///
/// This won't change the page, but will fire a `popstate` event.
pub fn replace(url: &str) -> Result<(), PushUrlError> {
    check_url(url)?;
    history()
        .replace_state_with_url(&JsValue::NULL, "", Some(url))
        .unwrap_throw();
    Ok(())
}

fn check_url(url: &str) -> Result<(), PushUrlError> {
    // Get the current url from web_sys
    let current = window().location().href().unwrap_throw();
    let current = match Url::parse(&current) {
        Ok(o) => o,
        Err(e) => wasm_bindgen::throw_str(&format!("cannot parse current origin: {}", e)),
    };
    // check `url` is actually a url, and matches the current one.
    match Url::parse(url) {
        Ok(u) => {
            if u.origin() == current.origin() {
                Ok(())
            } else {
                Err(PushUrlError::IncorrectOrigin { current, pushed: u })
            }
        }
        // if there is no base then allow
        Err(url::ParseError::RelativeUrlWithoutBase) => Ok(()),
        // unexpected error
        Err(e) => Err(PushUrlError::InvalidUrl(url.to_owned(), e)),
    }
}

// errors

/// The origin of a pushed url didn't match the current origin.
#[derive(Debug)]
pub enum PushUrlError {
    IncorrectOrigin { pushed: Url, current: Url },
    InvalidUrl(String, url::ParseError),
}

impl fmt::Display for PushUrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PushUrlError::*;
        match self {
            IncorrectOrigin { pushed, current } => write!(
                f,
                r#"the pushed url has origin "{}", but the current page has origin "{}""#,
                pushed, current,
            ),
            InvalidUrl(url, err) => write!(f, r#"could not parse url "{}": {}"#, url, err),
        }
    }
}

impl std::error::Error for PushUrlError {}

// utility

fn history() -> web_sys::History {
    window().history().unwrap_throw()
}

fn window() -> web_sys::Window {
    web_sys::window().unwrap_throw()
}
