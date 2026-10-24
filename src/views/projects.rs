use dioxus::prelude::*;

use crate::components::ProjectCard;
use crate::views::content;

#[component]
pub fn Projects() -> Element {
    let projects = content::all_projects();

    rsx! {
        div { id: "projects",
            h1 { class: "page-header", "Projects" }
            div { class: "projects-holder",
                {projects.iter().map(|project| {
                    rsx! {
                        ProjectCard { key: "{project.slug}", project: project.clone() }
                    }
                })}
            }
        }
    }
}
