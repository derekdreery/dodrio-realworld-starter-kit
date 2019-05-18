use dodrio::{Node, RenderContext};

/// <nav class="navbar navbar-light">
///   <div class="container">
///     <a class="navbar-brand" href="index.html">conduit</a>
///     <ul class="nav navbar-nav pull-xs-right">
///       <li class="nav-item">
///         <!-- Add "active" class when you're on that page" -->
///         <a class="nav-link active" href="">Home</a>
///       </li>
///       <li class="nav-item">
///         <a class="nav-link" href="">
///           <i class="ion-compose"></i>&nbsp;New Post
///         </a>
///       </li>
///       <li class="nav-item">
///         <a class="nav-link" href="">
///           <i class="ion-gear-a"></i>&nbsp;Settings
///         </a>
///       </li>
///       <li class="nav-item">
///         <a class="nav-link" href="">Sign up</a>
///       </li>
///     </ul>
///   </div>
/// </nav>
pub fn header<'a>(ctx: &mut RenderContext<'a>) -> Node<'a> {
    use dodrio::builder::*;
    nav(&ctx)
        .attr("class", "navbar navbar-light")
        .children([div(&ctx)
            .attr("class", "container")
            .children([
                a(&ctx)
                    .attr("class", "navbar-brand")
                    .attr("href", "#/")
                    .children([text("conduit")])
                    .finish(),
                ul(&ctx)
                    .attr("class", "nav navbar-nav pull-xs-right")
                    .children([
                        li(&ctx)
                            .attr("class", "nav-item")
                            .children([a(&ctx)
                                .attr("class", "nav-link")
                                .attr("href", "")
                                .children([
                                    i(&ctx).attr("class", "ion-compose").finish(),
                                    text("\u{00a0}New Post"), // non-breaking space
                                ])
                                .finish()])
                            .finish(),
                        li(&ctx)
                            .attr("class", "nav-item")
                            .children([a(&ctx)
                                .attr("class", "nav-link")
                                .attr("href", "")
                                .children([
                                    i(&ctx).attr("class", "ion-gear-a").finish(),
                                    text("\u{00a0}Settings"),
                                ])
                                .finish()])
                            .finish(),
                        li(&ctx)
                            .attr("class", "nav-item")
                            .children([a(&ctx)
                                .attr("class", "nav-link")
                                .attr("href", "#/login")
                                .children([text("Sign up")])
                                .finish()])
                            .finish(),
                    ])
                    .finish(),
            ])
            .finish()])
        .finish()
}

/// <footer>
///   <div class="container">
///     <a href="/" class="logo-font">conduit</a>
///     <span class="attribution">
///       An interactive learning project from <a href="https://thinkster.io">Thinkster</a>. Code &amp; design licensed under MIT.
///     </span>
///   </div>
/// </footer>
pub fn footer<'a>(ctx: &mut RenderContext<'a>) -> Node<'a> {
    use dodrio::builder::*;
    footer(&ctx)
        .children([div(&ctx)
            .attr("class", "container")
            .children([
                a(&ctx)
                    .attr("href", "/")
                    .attr("class", "logo-font")
                    .children([text("conduit")])
                    .finish(),
                span(&ctx)
                    .attr("class", "attribution")
                    .children([
                        text("An interactive learning project from "),
                        a(&ctx)
                            .attr("href", "https://thinkster.io")
                            .children([text("thinkster")])
                            .finish(),
                        text(". Code & design licensed under MIT."),
                    ])
                    .finish(),
            ])
            .finish()])
        .finish()
}
