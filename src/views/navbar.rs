use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

const LINKEDIN_SVG: &str = r#"<svg viewBox="0 0 16 16" aria-hidden="true" width="32" height="32"><path fill="currentColor" d="M12.225 12.225h-1.778V9.44c0-.664-.012-1.519-.925-1.519-.926 0-1.068.724-1.068 1.47v2.834H6.676V6.498h1.707v.783h.024c.348-.594.996-.95 1.684-.925 1.802 0 2.135 1.185 2.135 2.728l-.001 3.14zM4.67 5.715a1.037 1.037 0 01-1.032-1.031c0-.566.466-1.032 1.032-1.032.566 0 1.031.466 1.032 1.032 0 .566-.466 1.032-1.032 1.032zm.889 6.51h-1.78V6.498h1.78v5.727zM13.11 2H2.885A.88.88 0 002 2.866v10.268a.88.88 0 00.885.866h10.226a.882.882 0 00.889-.866V2.865a.88.88 0 00-.889-.864z"/></svg>"#;

const GITHUB_SVG: &str = r#"<svg viewBox="0 0 16 16" aria-hidden="true" width="32" height="32"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>"#;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { class: "flex flex-col min-h-screen",
            header {
                nav { id: "navbar",
                    div { class: "internal-links",
                        Link { to: Route::Home {}, "Home" }
                        Link { to: Route::BlogList {}, "Blog" }
                    }
                    div { class: "social-links",
                        a {
                            href: "https://www.linkedin.com/in/edward-west-985398217/",
                            target: "_blank",
                            dangerous_inner_html: LINKEDIN_SVG,
                        }
                        a {
                            href: "https://github.com/ejwestdev",
                            target: "_blank",
                            dangerous_inner_html: GITHUB_SVG,
                        }
                    }
                }
            }

            main { class: "flex flex-col grow", Outlet::<Route> {} }

        }
    }
}
