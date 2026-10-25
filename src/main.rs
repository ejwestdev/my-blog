use dioxus::prelude::*;
use views::{Blog, Home, Navbar, Project, Projects};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/projects")]
        Projects {},
        #[route("/projects/:slug")]
        Project { slug: String },
        #[route("/blog/:id")]
        Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const FAVICON_SVG: Asset = asset!("/assets/favicon.svg");
const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon.png");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        document::eval(
            r#"(function () {
                if (window.__cssReadyWatch) return; // install the watcher only once
                window.__cssReadyWatch = true;
                var root = document.documentElement;
                var pending = 0;
                var watched = new Set();
                var observer = null;
                function reveal() {
                    root.classList.add('css-ready');
                    if (observer) observer.disconnect();
                }
                function watch(link) {
                    if (watched.has(link) || link.sheet) return; // already loaded and applied
                    watched.add(link);
                    pending++;
                    var done = function () {
                        link.removeEventListener('load', done);
                        link.removeEventListener('error', done);
                        if (--pending <= 0) reveal();
                    };
                    link.addEventListener('load', done);
                    link.addEventListener('error', done);
                }
                // Stylesheets injected after this effect runs (the normal case).
                observer = new MutationObserver(function (mutations) {
                    mutations.forEach(function (m) {
                        m.addedNodes.forEach(function (node) {
                            if (node.tagName === 'LINK' && node.rel === 'stylesheet') watch(node);
                        });
                    });
                });
                observer.observe(document.head, { childList: true });
                document.querySelectorAll('link[rel="stylesheet"]').forEach(watch);
                Promise.resolve().then(function () {
                    document.querySelectorAll('link[rel="stylesheet"]').forEach(watch);
                    if (pending === 0) reveal();
                });
            })();"#,
        );
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON, sizes: "48x48" }
        document::Link { rel: "icon", href: FAVICON_SVG, r#type: "image/svg+xml" }
        document::Link { rel: "apple-touch-icon", href: APPLE_TOUCH_ICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
