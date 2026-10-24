use crate::Route;
use dioxus::prelude::*;

use crate::components::Footer;
use crate::views::content;

#[component]
pub fn Home() -> Element {
    let mut posts = content::all_posts();
    posts.sort_by(|a, b| b.id.cmp(&a.id));

    rsx! {
        div { class: "home-intro",
            h1 { class: "page-header", "Hi, my name is Edward" }
            p { "I'm a Fullstack Software Engineer from Los Angeles, CA." }
            p {
                "I'm passionate about building software and learning about new technologies. "
                "My areas of focus are mobile development in Swift & Kotlin. I also have built "
                "backend APIs for production use. AWS Lambda, Postgres, EC2, and FastAPI is my "
                "go to tech stack for backend services."
            }

            h2 { class: "section-header", "Recent" }
            ul { class: "recent-list",
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

        Footer {}
    }
}
