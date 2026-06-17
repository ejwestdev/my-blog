use crate::components::Body;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col justify-center min-h-screen bg-[#16191b]", Body {} }
    }
}
