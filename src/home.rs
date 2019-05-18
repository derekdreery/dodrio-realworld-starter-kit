use crate::util::bump_str;
use chrono::prelude::*;
use dodrio::{
    bumpalo::{self, Bump},
    Node, RenderContext,
};
use wasm_bindgen::prelude::*;

/// <div class="home-page">
/// <div class="banner">
///   <div class="container">
///     <h1 class="logo-font">conduit</h1>
///     <p>A place to share your knowledge.</p>
///   </div>
/// </div>
/// <div class="container page">
///   <div class="row">
///     <div class="col-md-9">
///       <div class="feed-toggle">
///         <ul class="nav nav-pills outline-active">
///           <li class="nav-item">
///             <a class="nav-link disabled" href="">Your Feed</a>
///           </li>
///           <li class="nav-item">
///             <a class="nav-link active" href="">Global Feed</a>
///           </li>
///         </ul>
///       </div>
///       <div class="article-preview">
///         <div class="article-meta">
///           <a href="profile.html"><img src="http://i.imgur.com/Qr71crq.jpg" /></a>
///           <div class="info">
///             <a href="" class="author">Eric Simons</a>
///             <span class="date">January 20th</span>
///           </div>
///           <button class="btn btn-outline-primary btn-sm pull-xs-right">
///             <i class="ion-heart"></i> 29
///           </button>
///         </div>
///         <a href="" class="preview-link">
///           <h1>How to build webapps that scale</h1>
///           <p>This is the description for the post.</p>
///           <span>Read more...</span>
///         </a>
///       </div>
///       <div class="article-preview">
///         <div class="article-meta">
///           <a href="profile.html"><img src="http://i.imgur.com/N4VcUeJ.jpg" /></a>
///           <div class="info">
///             <a href="" class="author">Albert Pai</a>
///             <span class="date">January 20th</span>
///           </div>
///           <button class="btn btn-outline-primary btn-sm pull-xs-right">
///             <i class="ion-heart"></i> 32
///           </button>
///         </div>
///         <a href="" class="preview-link">
///           <h1>The song you won't ever stop singing. No matter how hard you try.</h1>
///           <p>This is the description for the post.</p>
///           <span>Read more...</span>
///         </a>
///       </div>
///     </div>
///     <div class="col-md-3">
///       <div class="sidebar">
///         <p>Popular Tags</p>
///         <div class="tag-list">
///           <a href="" class="tag-pill tag-default">programming</a>
///           <a href="" class="tag-pill tag-default">javascript</a>
///           <a href="" class="tag-pill tag-default">emberjs</a>
///           <a href="" class="tag-pill tag-default">angularjs</a>
///           <a href="" class="tag-pill tag-default">react</a>
///           <a href="" class="tag-pill tag-default">mean</a>
///           <a href="" class="tag-pill tag-default">node</a>
///           <a href="" class="tag-pill tag-default">rails</a>
///         </div>
///       </div>
///     </div>
///   </div>
/// </div>
/// </div>
pub fn page<'a>(ctx: &mut RenderContext<'a>) -> Node<'a> {
    use dodrio::builder::*;

    // todo the number of stories + 1
    let mut wide_col = Vec::with_capacity(3);
    wide_col.push(feed_toggle(ctx));
    wide_col.push(
        ArticlePreview {
            title: "How to build webapps that scale",
            author: "Eric Simons",
            date: DateTime::parse_from_rfc3339("2018-01-20T12:00:00.000Z").unwrap_throw(),
            description: "This is the description for the post.",
            likes: 29,
        }
        .render(ctx),
    );
    div(&ctx)
        .attr("class", "home-page")
        .children([
            banner(ctx),
            div(&ctx)
                .attr("class", "container page")
                .children([div(&ctx)
                    .attr("class", "row")
                    .children([
                        div(&ctx)
                            .attr("class", "col-md-9")
                            .children(wide_col)
                            .finish(),
                        div(&ctx)
                            .attr("class", "col-md-3")
                            .children([sidebar(
                                &[
                                    "programming",
                                    "javascript",
                                    "emberjs",
                                    "angularjs",
                                    "react",
                                    "mean",
                                    "node",
                                    "rails",
                                ],
                                ctx,
                            )])
                            .finish(),
                    ])
                    .finish()])
                .finish(),
        ])
        .finish()
}

/// Draw the right-hand sidebar.
fn sidebar<'a>(tags: &[impl AsRef<str>], ctx: &mut RenderContext<'a>) -> Node<'a> {
    use dodrio::{builder::*, bumpalo::collections::Vec};
    let mut tag_list = Vec::with_capacity_in(tags.len() + 1, ctx.bump);
    tag_list.push(p(&ctx).children([text("Popular Tags")]).finish());
    for tag in tags {
        let tag = bumpalo::format!(in ctx.bump, "{}", tag.as_ref()).into_bump_str();
        tag_list.push(
            a(&ctx)
                .attr("href", "/")
                .attr("class", "tag-pill tag-default")
                .children([text(tag)])
                .finish(),
        );
    }
    div(&ctx)
        .attr("class", "sidebar")
        .children(tag_list)
        .finish()
}

/// The top banner
fn banner<'a>(ctx: &mut RenderContext<'a>) -> Node<'a> {
    use dodrio::builder::*;
    div(&ctx)
        .attr("class", "banner")
        .children([div(&ctx)
            .attr("class", "container")
            .children([
                h1(&ctx)
                    .attr("class", "logo-font")
                    .children([text("conduit")])
                    .finish(),
                p(&ctx)
                    .children([text("A place to share your knowledge.")])
                    .finish(),
            ])
            .finish()])
        .finish()
}

fn feed_toggle<'a>(ctx: &mut RenderContext<'a>) -> Node<'a> {
    use dodrio::builder::*;
    div(&ctx)
        .attr("class", "feed-toggle")
        .children([ul(&ctx)
            .attr("class", "nav nav-pills outline-active")
            .children([
                li(&ctx)
                    .attr("class", "nav-item")
                    .children([a(&ctx)
                        .attr("class", "nav-link disabled")
                        .attr("href", "/")
                        .children([text("Your Feed")])
                        .finish()])
                    .finish(),
                li(&ctx)
                    .attr("class", "nav-item")
                    .children([a(&ctx)
                        .attr("class", "nav-link active")
                        .attr("href", "/")
                        .children([text("Global Feed")])
                        .finish()])
                    .finish(),
            ])
            .finish()])
        .finish()
}

#[derive(Debug, Copy, Clone)]
struct ArticlePreview<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub description: &'a str,
    date: DateTime<FixedOffset>,
    // I'm ambitious, we can overflow u32.
    likes: u64,
}

impl<'a> ArticlePreview<'a> {
    fn render<'b>(&self, ctx: &mut RenderContext<'b>) -> Node<'b> {
        use dodrio::builder::*;
        div(&ctx)
            .attr("class", "article-preview")
            .children([
                div(&ctx)
                    .attr("class", "article-meta")
                    .children([
                        a(&ctx)
                            .attr("href", "/")
                            .children([img(&ctx)
                                .attr("src", "http://i.imgur.com/Qr71crq.jpg")
                                .finish()])
                            .finish(),
                        div(&ctx)
                            .attr("class", "info")
                            .children([
                                a(&ctx)
                                    .attr("href", "/")
                                    .attr("class", "author")
                                    .children([text(bump_str(self.author, ctx.bump))])
                                    .finish(),
                                span(&ctx)
                                    .attr("class", "date")
                                    .children([text(bump_str(
                                        format_args!(
                                            "{}{}",
                                            self.date.format("%B %-d"),
                                            date_suffix(&self.date)
                                        ),
                                        ctx.bump,
                                    ))])
                                    .finish(),
                            ])
                            .finish(),
                        button(&ctx)
                            .attr("class", "btn btn-outline-primary btn-sm pull-xs-right")
                            .children([
                                i(&ctx).attr("class", "ion-heart").finish(),
                                text(
                                    &bumpalo::format!(in ctx.bump, " {}", self.likes)
                                        .into_bump_str(),
                                ),
                            ])
                            .finish(),
                    ])
                    .finish(),
                a(&ctx)
                    .attr("href", "#")
                    .attr("class", "preview-link")
                    .children([
                        h1(&ctx)
                            .children([text(bump_str(self.title, ctx.bump))])
                            .finish(),
                        p(&ctx)
                            .children([text(bump_str(self.description, ctx.bump))])
                            .finish(),
                        span(&ctx).children([text("Read more...")]).finish(),
                    ])
                    .finish(),
            ])
            .finish()
    }
}

fn date_suffix<Tz: TimeZone>(date: &DateTime<Tz>) -> &'static str {
    match date.day() % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}
