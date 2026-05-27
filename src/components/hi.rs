use dioxus::prelude::*;

#[component]
pub fn Hi() -> Element {
    rsx! {
        div { class: "flex items-center justify-center grow", "Hi" }
    }
}