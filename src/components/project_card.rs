use crate::views::content::Project;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn ProjectCard(project: Project) -> Element {
    let card = rsx! {
        article { class: "project-card",
            h2 { "{project.title}" }
            p { "{project.description}" }
        }
    };

    if let Some(link) = &project.link {
        rsx! {
            a { class: "project-link", href: "{link}", target: "_blank", {card} }
        }
    } else {
        rsx! {
            Link {
                class: "project-link",
                to: Route::Project { slug: project.slug.to_string() },
                {card}
            }
        }
    }
}
