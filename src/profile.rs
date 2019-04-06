use crate::State;
use chrono::prelude::*;
use dodrio::{
    bumpalo::{self, Bump},
    ListenerCallback, Node, Render,
};
use wasm_bindgen::{prelude::*, JsCast};

/// <div class="auth-page">
///   <div class="container page">
///     <div class="row">
///
///       <div class="col-md-6 offset-md-3 col-xs-12">
///         <h1 class="text-xs-center">Sign up</h1>
///         <p class="text-xs-center">
///           <a href="">Have an account?</a>
///         </p>
///
///         <ul class="error-messages">
///           <li>That email is already taken</li>
///         </ul>
///
///         <form>
///           <fieldset class="form-group">
///             <input class="form-control form-control-lg" type="text" placeholder="Your Name">
///           </fieldset>
///           <fieldset class="form-group">
///             <input class="form-control form-control-lg" type="text" placeholder="Email">
///           </fieldset>
///           <fieldset class="form-group">
///             <input class="form-control form-control-lg" type="password" placeholder="Password">
///           </fieldset>
///           <button class="btn btn-lg btn-primary pull-xs-right">
///             Sign up
///           </button>
///         </form>
///       </div>
///
///     </div>
///   </div>
/// </div>

#[derive(Debug)]
pub struct LoginPage {
    name_start: String,
    // is there a way to avoid (de)allocating strings when we swap from none to some etc.
    name_current: Option<String>,
    email_start: String,
    email_current: Option<String>,
    password_start: String,
    password_current: Option<String>,
    errors: Vec<String>,
}

impl LoginPage {
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        LoginPage {
            name_start: name.into(),
            name_current: None,
            email_start: email.into(),
            email_current: None,
            password_start: password.into(),
            password_current: None,
            errors: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.name_current = None;
        self.email_current = None;
        self.password_current = None;
        self.errors.clear();
    }

    pub fn name(&self) -> &str {
        self.name_current.as_ref().unwrap_or(&self.name_start)
    }

    pub fn set_name(&mut self, new_name: impl AsRef<str>) {
        self.name_start.clear();
        self.name_start.push_str(new_name.as_ref());
        self.name_current = None;
    }

    fn set_name_ephemeral(&mut self, new_name: impl AsRef<str>) {
        let new_name = new_name.as_ref();
        if new_name == self.name_start {
            self.name_current = None;
        } else {
            let mut new_current = self.name_current.take().unwrap_or(String::new());
            new_current.clear();
            new_current.push_str(new_name);
            self.name_current = Some(new_current);
        }
    }

    pub fn email(&self) -> &str {
        self.email_current.as_ref().unwrap_or(&self.email_start)
    }

    pub fn set_email(&mut self, new_email: impl AsRef<str>) {
        self.email_start.clear();
        self.email_start.push_str(new_email.as_ref());
        self.email_current = None;
    }

    fn set_email_ephemeral(&mut self, new_email: impl AsRef<str>) {
        let new_email = new_email.as_ref();
        if new_email == self.email_start {
            self.email_current = None;
        } else {
            let mut new_current = self.email_current.take().unwrap_or(String::new());
            new_current.clear();
            new_current.push_str(new_email);
            self.email_current = Some(new_current);
        }
    }

    pub fn password(&self) -> &str {
        self.password_current
            .as_ref()
            .unwrap_or(&self.password_start)
    }

    pub fn set_password(&mut self, new_password: impl AsRef<str>) {
        self.password_start.clear();
        self.password_start.push_str(new_password.as_ref());
        self.password_current = None;
    }

    fn set_password_ephemeral(&mut self, new_password: impl AsRef<str>) {
        let new_password = new_password.as_ref();
        if new_password == self.password_start {
            self.password_current = None;
        } else {
            let mut new_current = self.password_current.take().unwrap_or(String::new());
            new_current.clear();
            new_current.push_str(new_password);
            self.password_current = Some(new_current);
        }
    }
}

impl Default for LoginPage {
    fn default() -> Self {
        LoginPage::new("", "", "")
    }
}

impl Render for LoginPage {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::{builder::*, bumpalo::collections::Vec};

        let mut main_children = Vec::with_capacity_in(4, bump);
        main_children.push(
            h1(bump)
                .attr("class", "text-xs-center")
                .children([text("Sign up")])
                .finish(),
        );
        main_children.push(
            p(bump)
                .attr("class", "text-xs-center")
                .children([a(bump)
                    .attr("href", "#/")
                    .children([text("Have an account?")])
                    .finish()])
                .finish(),
        );
        if let Some(msg) = Some("An error occurred") {
            main_children.push(error_msgs(&[msg], bump));
        }
        main_children.push(
            form(bump)
                .children([
                    form_ctrl(
                        "text",
                        "Your Name",
                        &self.name(),
                        |root, vdom, event| {
                            let new_value = event
                                .target()
                                .unwrap_throw()
                                .unchecked_into::<web_sys::HtmlInputElement>()
                                .value();
                            let state = root.unwrap_mut::<State>();
                            state.login_page.set_name_ephemeral(new_value);
                            vdom.schedule_render();
                            event.prevent_default();
                        },
                        bump,
                    ),
                    form_ctrl(
                        "text",
                        "Email",
                        &self.email(),
                        |root, vdom, event| {
                            let new_value = event
                                .target()
                                .unwrap_throw()
                                .unchecked_into::<web_sys::HtmlInputElement>()
                                .value();
                            let state = root.unwrap_mut::<State>();
                            state.login_page.set_email_ephemeral(new_value);
                            vdom.schedule_render();
                            event.prevent_default();
                        },
                        bump,
                    ),
                    form_ctrl(
                        "password",
                        "Password",
                        &self.password(),
                        |root, vdom, event| {
                            let new_value = event
                                .target()
                                .unwrap_throw()
                                .unchecked_into::<web_sys::HtmlInputElement>()
                                .value();
                            let state = root.unwrap_mut::<State>();
                            state.login_page.set_password_ephemeral(new_value);
                            vdom.schedule_render();
                            event.prevent_default();
                        },
                        bump,
                    ),
                    button(bump)
                        .attr("class", "btn btn-lg btn-primary pull-xs-right")
                        .children([text("Sign up")])
                        .finish(),
                ])
                .finish(),
        );

        div(bump)
            .attr("class", "auth-page")
            .children([div(bump)
                .attr("class", "container page")
                .children([div(bump)
                    .attr("class", "row")
                    .children([div(bump)
                        .attr("class", "col-md-6 offset-md-3 col-xs-12")
                        .children(main_children)
                        .finish()])
                    .finish()])
                .finish()])
            .finish()
    }
}

// helper render methods
// ---------------------

fn form_ctrl<'a, 'bump>(
    r#type: &'a str,
    placeholder: &'a str,
    value: &'a str,
    on_change: impl 'static + Fn(&mut dyn dodrio::RootRender, dodrio::VdomWeak, web_sys::Event),
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    use dodrio::builder::*;
    fieldset(bump)
        .attr("class", "form-group")
        .children([input(bump)
            .attr("class", "form-control form-control-lg")
            .attr("type", r#type)
            .attr("placeholder", placeholder)
            .attr("value", value)
            .on("input", on_change)
            .finish()])
        .finish()
}

fn error_msgs<'a, 'bump>(msgs: &'_ [&'a str], bump: &'bump Bump) -> Node<'bump>
where
    'a: 'bump,
{
    use dodrio::builder::*;
    use dodrio::bumpalo::collections::Vec;

    let mut msgs_bump = Vec::with_capacity_in(msgs.len(), bump);
    for msg in msgs {
        msgs_bump.push(li(bump).children([text(msg)]).finish());
    }
    ul(bump)
        .attr("class", "error-messages")
        .children(msgs_bump)
        .finish()
}
