use dioxus::prelude::*;

use crate::views::blog::all_blog_meta;
use crate::Route;

#[component]
pub fn Blogs() -> Element {
    let posts = all_blog_meta();

    rsx! {
        div { class: "flex flex-row gap-4",
            for post in posts {
                div { class: "w-fit",
                    Link {
                        to: Route::Blog { id: post.id },
                        class: "group gap-1",
                        display: "flex",
                        h2 { class: "underline underline-offset-4 decoration-white group-hover:no-underline text-gray-500",
                            "00{post.id}"
                        }
                        h2 { class: "no-underline group-hover:underline group-hover:underline-offset-4 transition-colors text-white",
                            "{post.title}"
                        }
                    }
                }
            }
        }
    }
}