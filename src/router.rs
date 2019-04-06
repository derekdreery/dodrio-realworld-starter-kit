use std::{fmt, str::FromStr};

/// Our route. Can be converted from and to a hash location.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Route {
    Home,
    Login,
    Register,
    Editor {
        slug: Option<String>,
    },
    Article {
        // todo check type
        id: u32,
    },
    Settings,
    ProfileFavourites {
        username: String,
    },
    Profile {
        username: String,
    },
}

pub struct UnrecognisedRoute;

impl FromStr for Route {
    type Err = UnrecognisedRoute;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        // should start with '#'
        if !(raw.chars().next() == Some('#')) {
            return Err(UnrecognisedRoute);
        }
        let mut raw = raw.split('/').filter(|&part| part != "").skip(1);
        match raw.next() {
            None => Ok(Route::Home),
            Some("login") => end(Route::Login, raw),
            Some("register") => end(Route::Register, raw),
            Some("editor") => match raw.next() {
                Some(slug) => end(
                    Route::Editor {
                        slug: Some(slug.to_owned()),
                    },
                    raw,
                ),
                None => Ok(Route::Editor { slug: None }),
            },
            Some("article") => match raw.next() {
                Some(id) => {
                    let id = id.parse().map_err(|_| UnrecognisedRoute)?;
                    end(Route::Article { id }, raw)
                }
                None => Err(UnrecognisedRoute),
            },
            Some("settings") => end(Route::Settings, raw),
            Some(username) if username.starts_with('@') => {
                let username = username[1..].to_owned();
                match raw.next() {
                    Some("favorites") => end(Route::ProfileFavourites { username }, raw),
                    Some(_) => Err(UnrecognisedRoute),
                    None => Ok(Route::Profile { username }),
                }
            }
            Some(_) => Err(UnrecognisedRoute),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("#/")?;
        match self {
            Route::Home => Ok(()),
            Route::Login => f.write_str("login"),
            Route::Register => f.write_str("register"),
            Route::Editor { slug: Some(slug) } => write!(f, "editor/{}", slug),
            Route::Editor { slug: None } => f.write_str("editor"),
            Route::Article { id } => write!(f, "article/{}", id),
            Route::Settings => f.write_str("settings"),
            Route::ProfileFavourites { username } => write!(f, "@{}/favorites", username),
            Route::Profile { username } => write!(f, "@{}", username),
        }
    }
}

/// Check we are at the end of the iterator, and return val.
#[inline]
fn end<T>(val: T, mut rest: impl Iterator) -> Result<T, UnrecognisedRoute> {
    match rest.next() {
        Some(_) => Err(UnrecognisedRoute),
        None => Ok(val),
    }
}
