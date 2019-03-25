#![feature(bind_by_move_pattern_guards)]

mod home;
mod layout;
mod login;
mod router;
mod util;

use dodrio::{bumpalo::Bump, Node, Render};
use futures::Future;
use router::Route;
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Debug)]
struct State {
    route: Route,
}

impl Default for State {
    fn default() -> Self {
        State { route: Route::Home }
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
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        let page = match self.route {
            Route::Home => home::page(bump),
            Route::Login => login::page(bump),
        };
        use dodrio::builder::div;
        div(bump)
            .children([layout::header(bump), page, layout::footer(bump)])
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
    log::info!("Old state: {:?}", root_state);
    log::info!("Msg: {:?}", msg);
    let change = root_state.update(msg);
    log::info!("state changed: {}", change);
    log::info!("New state: {:?}", root_state);
    web_sys::console::group_end();
    if change {
        vdom.schedule_render();
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap_throw();

    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let body = document.body().unwrap_throw();

    let app = State::default();

    let vdom = dodrio::Vdom::new(body.as_ref(), app);

    // routing
    let vdom2 = vdom.weak();
    let on_hash_change = move || {
        let hash = util::hash();
        log::debug!("route change event: raw = {:?}", hash);
        if let Some(route) = hash.and_then(|val| val.parse().ok()) {
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
            util::set_hash(&Route::Home.to_string());
        }
    };
    on_hash_change();
    let on_hash_change = Closure::wrap(Box::new(on_hash_change) as Box<dyn FnMut()>);

    window
        .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
        .unwrap_throw();

    // run forever
    vdom.forget();
    on_hash_change.forget();
}
