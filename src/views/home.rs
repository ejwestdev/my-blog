use crate::Route;
use dioxus::prelude::*;
use std::cmp::Reverse;

use crate::views::content;

#[component]
pub fn Home() -> Element {
    let mut posts = content::all_posts();
    posts.sort_by_key(|k| Reverse(k.id));

    rsx! {
        div { class: "home-intro",
            p { class: "tagline", "Welcome" }
            h1 { "About me" }
            p {
                "I'm a product oriented developer and builder based in Los Angeles. I've worked on "
                "mobile, fullstack and backend development. Lately, I've been learning embedded "
                "engineering with Rust."
            }
            p { class: "muted", "This blog is where I share my projects and talk about what I'm up to." }

            h2 { "Recent" }
            ul { class: "recent-list",
                {posts.iter().map(|post| {
                    rsx! {
                        li {
                            span { class: "post-date", "{post.date}" }
                            Link { to: Route::Blog { id: post.id }, "{post.title}" }
                        }
                    }
                })}
            }
        }

    }
}
