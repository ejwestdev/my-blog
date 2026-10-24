use crate::views::content;
use crate::Route;
use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Blog(id: i32) -> Element {
    let post = content::get_post(id);

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blog",
            Link { to: Route::Home {}, class: "back-link", "← Back" }
            if let Some(post) = post {
                h1 { class: "page-header", "{post.title}" }
                p { class: "post-date mb-6", "{post.date}" }
                div {
                    class: "prose max-w-none leading-relaxed",
                    dangerous_inner_html: "{post.html}",
                }
            } else {
                p { "Blog post not found." }
            }
        }
    }
}
