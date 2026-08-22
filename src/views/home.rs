use dioxus::prelude::*;

use crate::components::{Footer, ProjectCard};
use crate::views::content;

const ME_IMG: Asset = asset!("/assets/me.jpg");

#[component]
pub fn Home() -> Element {
    let projects = content::all_projects();

    rsx! {
        div { class: "home-intro",
            div { class: "home-text",
                h1 { class: "page-header", "Hi, my name is Edward" }
                p { "I'm a Fullstack Software Engineer from Los Angeles, CA." }
                p {
                    "I'm passionate about building software and learning about new technologies. "
                    "My areas of focus are mobile development in Swift & Kotlin. I also have built "
                    "backend APIs for production use. AWS Lambda, Postgres, EC2, and FastAPI is my "
                    "go to tech stack for backend services."
                }
                h2 { "Projects" }
                div { class: "projects-holder",
                    {projects.iter().map(|project| {
                        rsx! {
                            ProjectCard { key: "{project.slug}", project: project.clone() }
                        }
                    })}
                }
            }
            div { class: "home-photo",
                img { src: ME_IMG, alt: "Me" }
            }
        }

        Footer {}
    }
}
