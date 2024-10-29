use dioxus::prelude::*;
#[component]
pub fn AboutUs() -> Element {
    rsx! {
        document::Title { "About us" }
        div { class: "p-2",
            "This is a project created to demonstrate the programming language Rust and it's capabilities"
            p { "It also teaches some paradigms concepts while relating them to Rust" }
        }
    }
}
