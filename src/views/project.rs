use crate::views::content;
use crate::Route;
use dioxus::prelude::*;

const PROJECT_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Project(slug: String) -> Element {
    let project = content::get_project(&slug);

    rsx! {
        document::Link { rel: "stylesheet", href: PROJECT_CSS }
        div { id: "project",
            Link { to: Route::Home {}, class: "back-link", "← Back to Projects" }
            if let Some(project) = project {
                h1 { class: "page-header", "{project.title}" }
                div { dangerous_inner_html: "{project.html}" }
            } else {
                p { "Project not found." }
            }
        }
    }
}
