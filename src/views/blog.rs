use dioxus::prelude::*;

use crate::views::content;
use crate::Route;

pub struct BlogMeta {
    pub id: i32,
    pub title: String,
    pub date: String,
}

pub fn all_blog_meta() -> Vec<BlogMeta> {
    content::all_posts()
        .into_iter()
        .map(|p| BlogMeta {
            id: p.id,
            title: p.title,
            date: p.date,
        })
        .collect()
}

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Blog(id: i32) -> Element {
    let post = content::get_post(id);

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blog",
            Link { to: Route::Home {}, class: "text-blue-400 hover:text-blue-300", "← Back to Home" }
            if let Some(post) = post {
                h1 { class: "text-3xl font-bold text-white mb-4", "{post.title}" }
                p { class: "text-gray-500 mb-6", "{post.date}" }
                div {
                    class: "prose prose-invert max-w-none text-gray-300 leading-relaxed",
                    dangerous_inner_html: "{post.html}",
                }
            } else {
                p { class: "text-gray-400", "Blog post not found." }
            }
        }
    }
}
