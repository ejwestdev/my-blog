use crate::Route;
use dioxus::prelude::*;

use crate::components::Footer;
use crate::views::content;

#[component]
pub fn Home() -> Element {
    let mut posts = content::all_posts();
    posts.sort_by(|a, b| b.id.cmp(&a.id));

    rsx! {
        section { class: "home-intro",
            p { class: "eyebrow", "Welcome" }
            h1 { class: "display-title", "About me" }

            div { class: "home-intro-body",
                p {
                    "I'm a product oriented developer and builder based in Los Angeles."                }
                    "I've worked on mobile, fullstack and backend development. Lately, I've been learning embedded engineering with Rust."
                p { class: "text-muted",
                    "This blog is where I share my projects and talk about what I'm up to."
                }
            }

            section { class: "recent-section",
                div { class: "recent-header",
                    h2 { class: "section-title", "Recent" }
                }
                ul { class: "recent-list",
                    {posts.iter().map(|post| {
                        rsx! {
                            li { key: "{post.id}",
                                div { class: "recent-item-main",
                                    span { class: "post-date", "{post.date}" }
                                    div {
                                        Link { class: "post-link", to: Route::Blog { id: post.id },
                                            "{post.title}"
                                        }
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }

        Footer {}
    }
}
