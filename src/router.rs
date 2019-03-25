use std::{fmt, str::FromStr};

/// Our route. Can be converted from and to a hash location.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Route {
    Home,
    Login,
}

pub struct UnrecognisedRoute;

impl FromStr for Route {
    type Err = UnrecognisedRoute;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        // should start with '#'
        if !(raw.chars().next() == Some('#')) {
            return Err(UnrecognisedRoute);
        }
        // cannot panic: see check above.
        let mut raw = raw[1..].split('/').filter(|&part| part != "");
        match raw.next() {
            None => Ok(Route::Home),
            Some("login") => end(Route::Login, raw),
            Some(_) => Err(UnrecognisedRoute),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("#/")?;
        match self {
            Route::Home => (),
            Route::Login => f.write_str("login")?,
        }
        Ok(())
    }
}

#[inline]
fn end<T>(val: T, mut rest: impl Iterator) -> Result<T, UnrecognisedRoute> {
    match rest.next() {
        Some(_) => Err(UnrecognisedRoute),
        None => Ok(val),
    }
}
