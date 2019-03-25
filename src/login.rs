use chrono::prelude::*;
use dodrio::{
    bumpalo::{self, Bump},
    Node,
};
use wasm_bindgen::prelude::*;

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
pub fn page<'bump>(bump: &'bump Bump) -> Node<'bump> {
    use dodrio::builder::*;

    fn form_ctrl<'bump>(
        r#type: &'static str,
        placeholder: &'static str,
        bump: &'bump Bump,
    ) -> Node<'bump> {
        fieldset(bump)
            .attr("class", "form-group")
            .children([input(bump)
                .attr("class", "form-control form-control-lg")
                .attr("type", r#type)
                .attr("placeholder", placeholder)
                .finish()])
            .finish()
    }

    div(bump)
        .attr("class", "auth-page")
        .children([div(bump)
            .attr("class", "container page")
            .children([div(bump)
                .attr("class", "row")
                .children([div(bump)
                    .attr("class", "col-md-6 offset-md-3 col-xs-12")
                    .children([
                        h1(bump)
                            .attr("class", "text-xs-center")
                            .children([text("Sign up")])
                            .finish(),
                        p(bump)
                            .attr("class", "text-xs-center")
                            .children([a(bump)
                                .attr("href", "#/")
                                .children([text("Have an account?")])
                                .finish()])
                            .finish(),
                        ul(bump)
                            .attr("class", "error-messages")
                            .children([li(bump)
                                .children([text("That email is already taken")])
                                .finish()])
                            .finish(),
                        form(bump)
                            .children([
                                form_ctrl("text", "Your Name", bump),
                                form_ctrl("text", "Email", bump),
                                form_ctrl("password", "Password", bump),
                                button(bump)
                                    .attr("class", "btn btn-lg btn-primary pull-xs-right")
                                    .children([text("Sign up")])
                                    .finish(),
                            ])
                            .finish(),
                    ])
                    .finish()])
                .finish()])
            .finish()])
        .finish()
}
