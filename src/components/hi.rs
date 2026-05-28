use dioxus::prelude::*;

#[component]
pub fn Hi() -> Element {
    rsx! {
        div { class: "", "Hi" }
    }
}