#![feature(bind_by_move_pattern_guards)]

mod home;
mod layout;
mod profile;
mod router;
mod util;

use dodrio::{Node, Render, RenderContext};
use futures::Future;
use router::Route;
use wasm_bindgen::{prelude::*, JsCast};

use crate::profile::LoginPage;

#[derive(Debug)]
struct State {
    route: Route,
    login_page: LoginPage,
}

impl Default for State {
    fn default() -> Self {
        State {
            route: Route::Home,
            login_page: LoginPage::default(),
        }
    }
}

impl State {
    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Msg::PushRoute(new_route) if new_route != self.route => {
                self.route = new_route;
                true
            }
            _ => false, // do nothing by default
        }
    }
}

impl Render for State {
    fn render<'a>(&self, ctx: &mut RenderContext<'a>) -> Node<'a> {
        use dodrio::builder::div;

        let page = match self.route {
            Route::Home => home::page(ctx),
            Route::Login => self.login_page.render(ctx),
            /*
            Route::Register => profile::register_page(bump),
            Route::Editor { slug } => article::editor_page(slug, bump),
            Route::Article { id } => article::page(id, bump),
            Route::Settings => profile::settings_page(bump),
            Route::ProfileFavorites { username } => profile::favorites_page(bump),
            Route::Profile { username } => profile::profile_page(bump),
            */
            _ => unimplemented!(),
        };
        div(&ctx)
            .children([layout::header(ctx), page, layout::footer(ctx)])
            .finish()
    }
}

/// Possible updates
#[derive(Debug, Clone)]
enum Msg {
    PushRoute(Route),
}

impl Msg {
    fn debug_name(&self) -> &'static str {
        match self {
            Msg::PushRoute(_) => "PushRoute",
        }
    }
}

/// Mutate the state based on the message, and handle re-rendering etc..
///
/// Also log update events to the console
fn update(msg: Msg, root: &mut dyn dodrio::RootRender, vdom: dodrio::VdomWeak) {
    let root_state = root.unwrap_mut::<State>();
    web_sys::console::group_collapsed_4(
        &format!("%cAction %c{}%c ", msg.debug_name()).into(),
        &"color: #999; font-weight: normal;".into(),
        &"color: black; font-weight: bold;".into(),
        &"font-weight: normal;".into(),
    );
    log::debug!("Old state: {:?}", root_state);
    log::debug!("Msg: {:?}", msg);
    let change = root_state.update(msg);
    log::debug!("state changed: {}", change);
    log::debug!("New state: {:?}", root_state);
    web_sys::console::group_end();
    if change {
        vdom.schedule_render();
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap_throw();

    let window = util::window();
    let document = util::document();
    let body = document.body().unwrap_throw();

    let app = State::default();

    let vdom = dodrio::Vdom::new(body.as_ref(), app);

    // routing
    let vdom2 = vdom.weak();
    let on_route = move |route| {
        log::debug!("route change event: {:?}", route);
        if let Ok(route) = route {
            wasm_bindgen_futures::spawn_local(
                vdom2
                    .with_component({
                        let vdom = vdom2.clone();
                        move |root| update(Msg::PushRoute(route), root, vdom)
                    })
                    .map_err(|err| log::error!("{}", err)),
            )
        } else {
            log::warn!("Unrecognised route -> redirecting to home");
            wasm_history::push("").unwrap();
        }
    };
    let router = router::Router::new(on_route);

    // run forever
    vdom.forget();
    router.forget();
}
