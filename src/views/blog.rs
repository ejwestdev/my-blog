use crate::views::content;
use crate::Route;
use dioxus::prelude::*;

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
pub fn BlogList() -> Element {
    let posts = all_blog_meta();

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blog-list",
            h1 { class: "page-header", "Blog" }
            ul {
                {posts.iter().map(|post| {
                    rsx! {
                        li { key: "{post.id}",
                            Link { to: Route::Blog { id: post.id }, "{post.title}" }
                            span { class: "post-date", " — {post.date}" }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn Blog(id: i32) -> Element {
    let post = content::get_post(id);

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div { id: "blog",
            Link { to: Route::BlogList {}, class: "back-link", "← Back to Blog" }
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
