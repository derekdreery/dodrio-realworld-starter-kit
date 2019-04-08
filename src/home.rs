use chrono::prelude::*;
use dodrio::{
    bumpalo::{self, Bump},
    Node,
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
pub fn page<'bump>(bump: &'bump Bump) -> Node<'bump> {
    use dodrio::builder::*;

    // todo the number of stories + 1
    let mut wide_col = Vec::with_capacity(3);
    wide_col.push(feed_toggle(bump));
    wide_col.push(
        ArticlePreview {
            title: "How to build webapps that scale",
            author: "Eric Simons",
            date: DateTime::parse_from_rfc3339("2018-01-20T12:00:00.000Z").unwrap_throw(),
            description: "This is the description for the post.",
            likes: 29,
        }
        .render(bump),
    );
    div(bump)
        .attr("class", "home-page")
        .children([
            banner(bump),
            div(bump)
                .attr("class", "container page")
                .children([div(bump)
                    .attr("class", "row")
                    .children([
                        div(bump)
                            .attr("class", "col-md-9")
                            .children(wide_col)
                            .finish(),
                        div(bump)
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
                                bump,
                            )])
                            .finish(),
                    ])
                    .finish()])
                .finish(),
        ])
        .finish()
}

/// Draw the right-hand sidebar.
fn sidebar<'a, 'bump>(tags: &'a [impl AsRef<str> + 'a], bump: &'bump Bump) -> Node<'bump>
where
    'a: 'bump,
{
    use dodrio::builder::*;
    let mut tag_list = Vec::with_capacity(tags.len() + 1);
    tag_list.push(p(bump).children([text("Popular Tags")]).finish());
    for tag in tags {
        tag_list.push(
            a(bump)
                .attr("href", "/")
                .attr("class", "tag-pill tag-default")
                .children([text(tag.as_ref())])
                .finish(),
        );
    }
    div(bump)
        .attr("class", "sidebar")
        .children(tag_list)
        .finish()
}

/// The top banner
fn banner<'bump>(bump: &'bump Bump) -> Node<'bump> {
    use dodrio::builder::*;
    div(bump)
        .attr("class", "banner")
        .children([div(bump)
            .attr("class", "container")
            .children([
                h1(bump)
                    .attr("class", "logo-font")
                    .children([text("conduit")])
                    .finish(),
                p(bump)
                    .children([text("A place to share your knowledge.")])
                    .finish(),
            ])
            .finish()])
        .finish()
}

fn feed_toggle<'bump>(bump: &'bump Bump) -> Node<'bump> {
    use dodrio::builder::*;
    div(bump)
        .attr("class", "feed-toggle")
        .children([ul(bump)
            .attr("class", "nav nav-pills outline-active")
            .children([
                li(bump)
                    .attr("class", "nav-item")
                    .children([a(bump)
                        .attr("class", "nav-link disabled")
                        .attr("href", "/")
                        .children([text("Your Feed")])
                        .finish()])
                    .finish(),
                li(bump)
                    .attr("class", "nav-item")
                    .children([a(bump)
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
    fn render<'bump>(&'_ self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::builder::*;
        div(bump)
            .attr("class", "article-preview")
            .children([
                div(bump)
                    .attr("class", "article-meta")
                    .children([
                        a(bump)
                            .attr("href", "/")
                            .children([img(bump)
                                .attr("src", "http://i.imgur.com/Qr71crq.jpg")
                                .finish()])
                            .finish(),
                        div(bump)
                            .attr("class", "info")
                            .children([
                                a(bump)
                                    .attr("href", "/")
                                    .attr("class", "author")
                                    .children([text(self.author)])
                                    .finish(),
                                span(bump)
                                    .attr("class", "date")
                                    .children([text(
                                        bumpalo::format!(in bump, "{}{}",
                                                         self.date.format("%B %-d"),
                                                         date_suffix(&self.date))
                                        .into_bump_str(),
                                    )])
                                    .finish(),
                            ])
                            .finish(),
                        button(bump)
                            .attr("class", "btn btn-outline-primary btn-sm pull-xs-right")
                            .children([
                                i(bump).attr("class", "ion-heart").finish(),
                                text(bumpalo::format!(in bump, " {}", self.likes).into_bump_str()),
                            ])
                            .finish(),
                    ])
                    .finish(),
                a(bump)
                    .attr("href", "#")
                    .attr("class", "preview-link")
                    .children([
                        h1(bump).children([text(self.title)]).finish(),
                        p(bump).children([text(self.description)]).finish(),
                        span(bump).children([text("Read more...")]).finish(),
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
